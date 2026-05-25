// core/tests.rs
#![allow(clippy::module_inception)]

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use rusqlite::Connection;
    use std::fs;
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    use crate::core::auth::{is_valid_host, is_valid_origin};
    use crate::core::backups::{restore_backup, save_backup};
    use crate::core::compiler::compile_static_site;
    use crate::core::db::*;
    use crate::core::detectors::run_detectors;
    use crate::core::guardrails::run_guardrails_check;
    use crate::core::migrations::{get_current_version, get_expected_version, run_migrations};

    // 1. Migration Tests
    #[test]
    fn test_migrations() {
        let mut conn = Connection::open_in_memory().unwrap();

        // Initial run should apply all migrations
        let res = run_migrations(&mut conn);
        assert!(res.is_ok(), "Migrations failed to execute: {:?}", res);

        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, get_expected_version());

        // Running migrations again should be a safe, clean no-op
        let res_noop = run_migrations(&mut conn);
        assert!(
            res_noop.is_ok(),
            "Second migration run failed: {:?}",
            res_noop
        );
        let version_after = get_current_version(&conn).unwrap();
        assert_eq!(version_after, get_expected_version());
    }

    // 2. Auth Tests
    #[test]
    fn test_auth_checks() {
        // Host checks
        assert!(is_valid_host("127.0.0.1:12053"));
        assert!(!is_valid_host("localhost:12053"));
        assert!(is_valid_host("  127.0.0.1:12053  ")); // Whitespace cleanup
        assert!(!is_valid_host("google.com"));
        assert!(!is_valid_host("127.0.0.1:8080"));

        // Origin checks
        assert!(is_valid_origin("chrome-extension://someuniqueextensionid"));
        assert!(!is_valid_origin("null"));
        assert!(!is_valid_origin("http://evilwebsite.com"));
        assert!(!is_valid_origin("https://localhost:12053"));
    }

    // 3. Detector Tests
    #[test]
    fn test_osint_detectors() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();

        // Setup test sources
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Brighton Town Council".to_string(),
                url: "https://brighton.gov/agenda.xml".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        // 3a. Test Money Threshold (fires > $250k)
        let ev_money_high = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://brighton.gov/agenda.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Approved contract for $350,000 road maintenance project.".to_string(),
                content_hash: "hash_money_high".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();

        let ev_money_low = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://brighton.gov/agenda.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Approved contract for $45,000 for park benches.".to_string(),
                content_hash: "hash_money_low".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();

        let profile_json = r#"{"money_threshold": 250000.0, "watchlist": ["John Doe"]}"#;

        let _new_leads =
            run_detectors(&conn, &[ev_money_high, ev_money_low], profile_json).unwrap();

        // Assert only the high amount lead was created (plus the New Primary Record lead which fires automatically for primary records)
        let leads = list_leads(&conn).unwrap();

        // We expect:
        // 1. "New Primary Record" for ev_money_high
        // 2. "Money Threshold" for ev_money_high
        // 3. "New Primary Record" for ev_money_low
        assert!(leads
            .iter()
            .any(|l| l.detector_name == "Money Threshold" && l.why.contains("$350,000")));
        assert!(!leads
            .iter()
            .any(|l| l.detector_name == "Money Threshold" && l.why.contains("$45,000")));

        // 3b. Test Watchlist Detector
        let ev_watchlist = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://brighton.gov/agenda.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Council met with contractor John Doe regarding landfill operations."
                    .to_string(),
                content_hash: "hash_watchlist".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();

        let _new_leads2 = run_detectors(&conn, &[ev_watchlist], profile_json).unwrap();
        let leads_after = list_leads(&conn).unwrap();
        assert!(leads_after
            .iter()
            .any(|l| l.detector_name == "Watchlist Hit" && l.why.contains("John Doe")));

        // 3c. Test Decision / Vote
        let ev_vote = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://brighton.gov/agenda.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "The board unanimously approved the zoning change request.".to_string(),
                content_hash: "hash_vote".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
        run_detectors(&conn, &[ev_vote], profile_json).unwrap();
        let leads_vote = list_leads(&conn).unwrap();
        assert!(leads_vote
            .iter()
            .any(|l| l.detector_name == "Decision / Vote"));
    }

    // 4. Guardrails Tests
    #[test]
    fn test_guardrails() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();

        // Insert a lead and link to evidence
        let lead_id = insert_lead(
            &conn,
            &Lead {
                id: None,
                detector_name: "Zoning Board".to_string(),
                why: "Zoning change".to_string(),
                confidence: "high".to_string(),
                risk_level: "low".to_string(),
                from_scan_lead_id: None,
                confirmation_checklist: "[]".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
            &[],
        )
        .unwrap();

        // Draft with missing citation (should warn) and accusatory term (should error)
        let draft_id = insert_draft(&conn, &Draft {
            id: None,
            lead_id: Some(lead_id),
            format: "story".to_string(),
            title: "Accusation Story".to_string(),
            content: "The mayor engaged in corrupt activities during yesterday's budget meeting.\n\nHe refused to answer public comment questions.".to_string(),
            status: "draft_generated".to_string(),
            verification_checklist: "[]".to_string(),
            missing_evidence_notes: None,
            correction_note: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }).unwrap();

        let report = run_guardrails_check(&conn, draft_id).unwrap();
        assert!(!report.is_clean);

        let has_accusatory_err = report
            .issues
            .iter()
            .any(|i| i.category == "Accusatory Language" && i.severity == "error");
        let has_citation_warn = report
            .issues
            .iter()
            .any(|i| i.category == "Citation Coverage" && i.severity == "warning");
        assert!(
            has_accusatory_err,
            "Should have failed on accusatory term without citation"
        );
        assert!(
            has_citation_warn,
            "Should have warned on paragraph missing citation link"
        );

        // Legal naming presumption of innocence warning test
        let draft_id2 = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: Some(lead_id),
                format: "story".to_string(),
                title: "Arrest Story".to_string(),
                content: "Police arrested a clerk for embezzlement at city hall (evidence:101)."
                    .to_string(), // Has citation, but lacks "alleged"
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();

        let report2 = run_guardrails_check(&conn, draft_id2).unwrap();
        assert!(!report2.is_clean);
        let has_presumption_err = report2
            .issues
            .iter()
            .any(|i| i.category == "Legal Naming" && i.severity == "error");
        assert!(
            has_presumption_err,
            "Should trigger error due to missing 'alleged' modifier"
        );
    }

    // 5. Backups Tests
    #[test]
    fn test_backups() {
        let temp_dir = tempdir().unwrap();
        let db_file_path = temp_dir.path().join("live.db");
        let backup_file_path = temp_dir.path().join("backup.db");

        // 1. Initialize live DB and write some records
        let conn = init_db(db_file_path.to_str().unwrap()).unwrap();
        insert_source(
            &conn,
            &Source {
                id: None,
                name: "Original Source".to_string(),
                url: "https://orig.gov".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        // 2. Save Backup
        save_backup(&conn, backup_file_path.to_str().unwrap()).unwrap();
        assert!(backup_file_path.exists());

        // 3. Mutate live DB (insert new item)
        insert_source(
            &conn,
            &Source {
                id: None,
                name: "Mutated Source".to_string(),
                url: "https://mutate.gov".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        let sources_mutated = list_sources(&conn).unwrap();
        assert_eq!(sources_mutated.len(), 2);

        // 4. Restore DB
        let db_conn_arc = Arc::new(Mutex::new(conn));
        restore_backup(
            &db_conn_arc,
            backup_file_path.to_str().unwrap(),
            db_file_path.to_str().unwrap(),
        )
        .unwrap();

        // 5. Verify restored state (should only have the original source)
        {
            let conn_after = db_conn_arc.lock().unwrap();
            let sources_restored = list_sources(&conn_after).unwrap();
            assert_eq!(sources_restored.len(), 1);
            assert_eq!(sources_restored[0].name, "Original Source");
        }

        // 6. Test Corrupt File Restore (Should reject and keep live DB safe)
        let corrupt_path = temp_dir.path().join("corrupt.db");
        fs::write(&corrupt_path, b"garbage text data").unwrap();

        let restore_res = restore_backup(
            &db_conn_arc,
            corrupt_path.to_str().unwrap(),
            db_file_path.to_str().unwrap(),
        );
        assert!(restore_res.is_err());

        // Assert live DB remains intact
        {
            let conn_final = db_conn_arc.lock().unwrap();
            let sources_final = list_sources(&conn_final).unwrap();
            assert_eq!(sources_final.len(), 1);
            assert_eq!(sources_final[0].name, "Original Source");
        }
    }

    // 6. Compiler and Site Gen Tests (Milestone integration test)
    #[test]
    fn test_compiler_static_site() {
        let temp_dir = tempdir().unwrap();
        let live_db_path = temp_dir.path().join("live.db");
        let site_output_path = temp_dir.path().join("dist");

        let conn = init_db(live_db_path.to_str().unwrap()).unwrap();

        // Setup test records
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Planning Commission".to_string(),
                url: "https://planning.gov".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        let ev_id = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://planning.gov/minutes_12.html".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Commission approved zoning change request for building C.".to_string(),
                content_hash: "hash_commission".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();

        let lead_id = insert_lead(
            &conn,
            &Lead {
                id: None,
                detector_name: "Decision / Vote".to_string(),
                why: "Approved zoning change for building C.".to_string(),
                confidence: "high".to_string(),
                risk_level: "low".to_string(),
                from_scan_lead_id: None,
                confirmation_checklist: "[]".to_string(),
                created_at: Utc::now().to_rfc3339(),
            },
            &[ev_id],
        )
        .unwrap();

        // Create published draft
        let draft_id = insert_draft(&conn, &Draft {
            id: None,
            lead_id: Some(lead_id),
            format: "investigation".to_string(), // maps to 'stories' subfolder
            title: "Zoning Board Approves New Building C Expansion".to_string(),
            content: "In yesterday's meeting, the commission formally approved zoning request for [Building C](evidence:1).\n\nThis marks a significant expansion plan.".to_string(),
            status: "published".to_string(), // MUST be published to compile
            verification_checklist: "[]".to_string(),
            missing_evidence_notes: None,
            correction_note: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }).unwrap();

        let profile_json = r#"{"site_title": "Local Observer", "site_subtitle": "Evidence first", "about_text": "About Observer", "ethics_text": "Ethics", "how_we_report_text": "How We Report"}"#;

        // Compile
        compile_static_site(&conn, site_output_path.to_str().unwrap(), profile_json).unwrap();

        // Assert files are written correctly
        assert!(site_output_path.join("styles.css").exists());
        assert!(site_output_path.join("print.css").exists());
        assert!(site_output_path.join("index.html").exists());
        assert!(site_output_path.join("about.html").exists());
        assert!(site_output_path.join("ethics.html").exists());
        assert!(site_output_path.join("how-we-report.html").exists());
        assert!(site_output_path.join("corrections.html").exists());
        assert!(site_output_path.join("feed.xml").exists());

        // Assert story subfolder file exists
        let post_html_path = site_output_path
            .join("stories")
            .join(format!("{}.html", draft_id));
        assert!(
            post_html_path.exists(),
            "Compiled post HTML path missing: {:?}",
            post_html_path
        );

        // Read compiled post contents and verify citations have been converted to anchors
        let content = fs::read_to_string(post_html_path).unwrap();
        assert!(
            content.contains("Local Observer"),
            "Site title placeholder fail"
        );
        assert!(
            content.contains("href=\"#evidence-1\""),
            "Citation replacement failed! Href is not pointing to evidence anchor."
        );
        assert!(
            content.contains("id=\"evidence-1\""),
            "Citations section missing evidence ID anchor tag."
        );
    }

    // 7. DuckDuckGo Auto-Discovery HTML Parser Test
    #[test]
    fn test_parse_duckduckgo_html() {
        use crate::core::discovery::parse_duckduckgo_html;
        let mock_html = r#"
            <html>
            <body>
            <div class="result">
                <h2 class="result__title">
                    <a class="result__a" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fwww.brightonco.gov%2Fagendacenter&amp;rut=123">Agenda Center • Brighton, CO</a>
                </h2>
            </div>
            <div class="result">
                <h2 class="result__title">
                    <a class="result__a" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Freddit.com%2Fr%2Fbrightonco&amp;rut=456"><b>Brighton</b> Reddit</a>
                </h2>
            </div>
            </body>
            </html>
        "#;
        let results = parse_duckduckgo_html(mock_html).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "Agenda Center • Brighton, CO");
        assert_eq!(results[0].1, "https://www.brightonco.gov/agendacenter");
        assert_eq!(results[1].0, "Brighton Reddit");
        assert_eq!(results[1].1, "https://reddit.com/r/brightonco");
    }
    #[test]
    fn test_compiler_xss_safe() {
        let conn = init_db("file:test_compiler_xss_safe?mode=memory&cache=shared").unwrap();
        let temp_dir = tempdir().unwrap();
        assert!("&lt;script&gt;".contains("&lt;script"));

        // Use insert_source so we have a source for the lead
        let _source_id = crate::core::db::insert_source(
            &conn,
            &crate::core::db::Source {
                id: None,
                name: "Test Source".to_string(),
                url: "<script>alert(1)</script>".to_string(), // Source URL with XSS
                r#type: "rss".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        let lead_id = crate::core::db::insert_lead(
            &conn,
            &crate::core::db::Lead {
                id: None,
                detector_name: "test".to_string(),
                why: "<script>alert(1)</script>".to_string(),
                confidence: "high".to_string(),
                risk_level: "low".to_string(),
                from_scan_lead_id: None,
                confirmation_checklist: "[]".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
            },
            &[],
        )
        .unwrap();

        let draft_id = crate::core::db::insert_draft(
            &conn,
            &crate::core::db::Draft {
                id: None,
                lead_id: Some(lead_id),
                format: "story".to_string(),
                title: "<script>alert(1)</script>".to_string(),
                content: "Hello <script>alert(1)</script>".to_string(),
                status: "published".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: Some("<script>alert(1)</script>".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            },
        )
        .unwrap();

        crate::core::db::insert_published_post(
            &conn,
            &crate::core::db::PublishedPost {
                id: None,
                draft_id,
                file_path: "stories/1.html".to_string(),
                url: "http://example.com/stories/1.html".to_string(),
                correction_history: String::new(),
                published_at: chrono::Utc::now().to_rfc3339(),
            },
        )
        .unwrap();

        crate::core::compiler::compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}")
            .unwrap();
        let html = std::fs::read_to_string(temp_dir.path().join("index.html")).unwrap();

        // Assertions
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script"));
        assert!(!html.contains("onerror="));

        let html2 = std::fs::read_to_string(temp_dir.path().join("stories/1.html")).unwrap();
        assert!(html2.contains("&lt;script&gt;"));
        assert!(!html2.contains("<script"));
        assert!(!html2.contains("onerror="));
    }

    #[test]
    fn test_settings_round_trip() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();

        // set value
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            ["test_key", "test_value_1"],
        )
        .unwrap();
        let val1: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'test_key'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(val1, "test_value_1");

        // overwrite value
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            ["test_key", "test_value_2"],
        )
        .unwrap();
        let val2: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'test_key'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(val2, "test_value_2");
    }

    // 8. Phase 3 Diagnostics Tests
    #[tokio::test]
    async fn test_gather_diagnostics_has_all_fields() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let db_conn = Arc::new(Mutex::new(conn));
        let temp_dir = tempdir().unwrap();

        let diags =
            crate::core::diagnostics::gather_diagnostics(&db_conn, temp_dir.path().to_path_buf())
                .await
                .unwrap();
        assert!(!diags.app_version.is_empty());
        assert!(!diags.os_name.is_empty());
        assert!(!diags.os_version.is_empty());
        assert!(!diags.tauri_version.is_empty());
        assert_eq!(diags.db_schema_version, get_expected_version() as i64);
        assert_eq!(diags.evidence_count, 0);
        assert_eq!(diags.leads_count, 0);
        assert_eq!(diags.drafts_count, 0);
        assert_eq!(diags.published_posts_count, 0);
    }

    #[tokio::test]
    async fn test_export_diagnostics_writes_valid_json() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let db_conn = Arc::new(Mutex::new(conn));
        let temp_dir = tempdir().unwrap();

        let file_path = temp_dir.path().join("diag.json");
        crate::tauri_cmds::export_diagnostics_inner(
            &db_conn,
            temp_dir.path().to_path_buf(),
            file_path.clone(),
        )
        .await
        .unwrap();

        let content = std::fs::read_to_string(&file_path).unwrap();
        let parsed: crate::core::diagnostics::Diagnostics = serde_json::from_str(&content).unwrap();
        assert!(!parsed.app_version.is_empty());
        assert!(!parsed.os_name.is_empty());
    }

    #[test]
    fn test_export_diagnostics_path_validation_rejects_traversal() {
        let temp_dir = tempdir().unwrap();
        let app_data = temp_dir.path().join("app_data");
        let downloads = temp_dir.path().join("downloads");
        std::fs::create_dir_all(&app_data).unwrap();
        std::fs::create_dir_all(&downloads).unwrap();

        // Good path in app_data
        let good_path = app_data.join("export.json");
        let res = crate::tauri_cmds::validate_export_path(
            app_data.clone(),
            downloads.clone(),
            good_path.to_str().unwrap(),
        );
        assert!(res.is_ok());

        // Good path in downloads
        let good_path2 = downloads.join("export2.json");
        let res2 = crate::tauri_cmds::validate_export_path(
            app_data.clone(),
            downloads.clone(),
            good_path2.to_str().unwrap(),
        );
        assert!(res2.is_ok());

        // Bad path using traversal
        let bad_path = app_data.join("..").join("etc").join("passwd");
        // Ensure parent directory for bad path exists so canonicalize doesn't fail early
        let etc_dir = temp_dir.path().join("etc");
        std::fs::create_dir_all(&etc_dir).unwrap();

        let res_bad = crate::tauri_cmds::validate_export_path(
            app_data.clone(),
            downloads.clone(),
            bad_path.to_str().unwrap(),
        );
        assert!(res_bad.is_err());
        assert_eq!(res_bad.unwrap_err(), "Path is outside allowed directories");
    }
}
