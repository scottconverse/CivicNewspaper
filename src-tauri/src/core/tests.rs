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

            // C-2 regression: foreign-key enforcement is per-connection and must
            // survive a restore (the live handle is reopened during restore).
            let fk_on: i64 = conn_after
                .query_row("PRAGMA foreign_keys;", [], |r| r.get(0))
                .unwrap();
            assert_eq!(fk_on, 1, "foreign_keys must be ON after restore (C-2)");
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
    fn test_compiler_xss_safe_profile_fields() {
        // ENG-002: author-controlled CommunityProfile fields (site_title,
        // site_subtitle, about_text) are interpolated into every generated page
        // and the RSS feed. They must be entity-encoded in all sinks, not just
        // the draft-derived fields covered by test_compiler_xss_safe (which uses
        // an empty profile and never exercises these paths).
        let conn = init_db("file:test_compiler_xss_safe_profile?mode=memory&cache=shared").unwrap();
        let temp_dir = tempdir().unwrap();

        let profile_json = r#"{
            "site_title": "<script>alert('title')</script>",
            "site_subtitle": "<img src=x onerror=alert('sub')>",
            "about_text": "<script>alert('about')</script>",
            "ethics_text": "ok",
            "how_we_report_text": "ok"
        }"#;

        crate::core::compiler::compile_static_site(
            &conn,
            temp_dir.path().to_str().unwrap(),
            profile_json,
        )
        .unwrap();

        // index.html carries site_title/site_subtitle (header) and about_text (sidebar).
        let index_html = std::fs::read_to_string(temp_dir.path().join("index.html")).unwrap();
        assert!(
            index_html.contains("&lt;script&gt;"),
            "profile fields must be entity-encoded in index.html"
        );
        assert!(
            !index_html.contains("<script"),
            "no live <script> tag may form in index.html"
        );
        assert!(
            !index_html.contains("<img"),
            "no live <img> tag may form in index.html"
        );

        // feed.xml carries site_title/site_subtitle in <title>/<description>.
        let feed_xml = std::fs::read_to_string(temp_dir.path().join("feed.xml")).unwrap();
        assert!(
            !feed_xml.contains("<script"),
            "no live <script> tag may form in feed.xml"
        );
        assert!(
            !feed_xml.contains("<img"),
            "no live <img> tag may form in feed.xml"
        );

        // corrections.html and the info pages also interpolate site_title/subtitle.
        let corrections_html =
            std::fs::read_to_string(temp_dir.path().join("corrections.html")).unwrap();
        assert!(
            !corrections_html.contains("<script"),
            "no live <script> tag may form in corrections.html"
        );
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
    // 9. Phase 4 Tests
    #[test]
    fn test_source_tier_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        // Just run migrations and ensure they pass without error
        run_migrations(&mut conn).unwrap();
        // Insert a source to test constraints
        let res = conn.execute("INSERT INTO sources (name, url, type, tier) VALUES ('Test', 'http', 'rss', 'invalid_tier')", []);
        assert!(res.is_err(), "Should fail constraint check");
    }

    #[test]
    fn test_source_tier_backfill_media_lead() {
        let conn = Connection::open_in_memory().unwrap();
        // Create table up to 0005
        conn.execute_batch(include_str!("../../migrations/0001_init.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0003_settings.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0004_source_tier.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0005_daily_scans.sql"))
            .unwrap();

        // Backfill a legacy row
        conn.execute("INSERT INTO sources (name, url, type, tier) VALUES ('Legacy', 'http', 'rss', 'community_signal')", []).unwrap();

        // Run latest migrations
        conn.execute_batch(include_str!(
            "../../migrations/0006_daily_scan_lead_source_nullable.sql"
        ))
        .unwrap();
        conn.execute_batch(include_str!("../../migrations/0007_source_tier_check.sql"))
            .unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM sources", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_list_prompts_returns_bundled() {
        let prompts = crate::core::prompts::list_prompts();
        assert!(!prompts.is_empty());
    }

    #[test]
    fn test_get_prompt_loads_aggregator() {
        let content = std::fs::read_to_string("prompts/aggregator.md").unwrap();
        assert!(content.contains("original_url"));
    }

    #[test]
    fn test_daily_scan_parses_fixture_response() {
        let response = r#"
        {
          "leads": [
            {
              "title": "Topic",
              "summary": "Sum",
              "original_url": "http"
            }
          ]
        }
        "#;
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        conn.execute(
            "INSERT INTO daily_scan_runs (started_at, run_status) VALUES ('', 'running')",
            [],
        )
        .unwrap();
        conn.execute("INSERT INTO sources (name, url, type, tier) VALUES ('Test', 'http', 'rss', 'community_signal')", []).unwrap();
        crate::core::daily_scan::parse_and_save_scan_response(&conn, 1, response).unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM daily_scan_leads", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: exercises the real model-selection path
    // (get_selected_model_or_fallback) plus the core rewrite logic with an
    // injected fake client, so no Tauri mock_app() is needed (P5-003 resolved).
    #[tokio::test]
    async fn test_plain_language_rewrite_invokes_ollama() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        // Insert selected model setting; get_selected_model_or_fallback returns
        // it directly (no network) when present.
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('model.selected', 'fake-model')",
            [],
        )
        .unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        let model = crate::tauri_cmds::get_selected_model_or_fallback(&db_conn).await;
        assert_eq!(model, "fake-model");

        struct FakeLlmClient;
        #[async_trait::async_trait]
        impl crate::core::llm::LlmClient for FakeLlmClient {
            async fn call(
                &self,
                model: &str,
                prompt: &str,
                system: &str,
            ) -> Result<String, String> {
                assert_eq!(model, "fake-model");
                assert!(prompt.contains("Rewrite this"));
                assert!(system.contains("summarizer"));
                Ok("Rewritten text".to_string())
            }
        }
        let llm_client: Arc<dyn crate::core::llm::LlmClient> = Arc::new(FakeLlmClient);

        let res =
            crate::core::llm::plain_language_rewrite(&llm_client, &model, "Hello", "story").await;

        assert_eq!(res.unwrap(), "Rewritten text");
    }

    #[test]
    fn test_prompt_schema_drift() {
        let content = std::fs::read_to_string("prompts/aggregator.md").unwrap();
        // Extract the fenced JSON example block
        let start_idx = content
            .find("```json")
            .expect("Missing ```json in aggregator.md");
        let after_start = &content[start_idx + 7..];
        let end_idx = after_start
            .find("```")
            .expect("Missing closing ``` in aggregator.md");
        let json_str = &after_start[..end_idx].trim();

        let scan_result: crate::core::daily_scan::ScanResult = serde_json::from_str(json_str)
            .expect("Failed to deserialize JSON block from aggregator.md");
        assert!(
            !scan_result.leads.is_empty(),
            "Leads vector should not be empty"
        );

        let lead = &scan_result.leads[0];
        assert!(!lead.title.is_empty(), "Title should not be empty");
        assert!(!lead.summary.is_empty(), "Summary should not be empty");
        assert!(
            !lead.original_url.is_empty(),
            "Original URL should not be empty"
        );
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: start_for_test spawns the bundled fixture without a
    // Tauri AppHandle, so no mock_app() is constructed (P5-004 resolved).
    #[test]
    fn test_ollama_sidecar_spawns_a_live_child() {
        // Reserve an OS-assigned port, then release it, so the collision probe
        // sees a free port and start_for_test proceeds to spawn the fixture.
        let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let free_addr = probe.local_addr().unwrap().to_string();
        drop(probe);

        let sidecar = crate::core::llm::OllamaSidecar::new();
        assert!(sidecar.child.lock().unwrap().is_none());

        let res = sidecar.start_for_test(&free_addr);
        assert!(res.is_ok());

        {
            let guard = sidecar.child.lock().unwrap();
            assert!(guard.is_some());
            let p = guard.as_ref().unwrap().pid();
            assert!(p > 0);
        }

        let res_stop = sidecar.stop();
        assert!(res_stop.is_ok());
        assert!(sidecar.child.lock().unwrap().is_none());
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Cross-platform: no mock_app().
    #[test]
    fn test_ollama_sidecar_terminates_cleanly_on_drop() {
        let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let free_addr = probe.local_addr().unwrap().to_string();
        drop(probe);

        // test calls sidecar.stop() via drop
        let pid = {
            let sidecar = crate::core::llm::OllamaSidecar::new();
            let res = sidecar.start_for_test(&free_addr);
            assert!(res.is_ok());

            let guard = sidecar.child.lock().unwrap();
            assert!(guard.is_some());
            let p = guard.as_ref().unwrap().pid();
            assert!(p > 0);
            p
        };

        // At this point, the sidecar has been dropped, so the process should
        // have been terminated. Verify drop implicitly calls sidecar.stop().
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let process_exists = sys.process(sysinfo::Pid::from(pid as usize)).is_some();
        assert!(
            !process_exists,
            "Sidecar process should be terminated after drop"
        );
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: calls the core scan directly with an injected fake
    // client, so no Tauri mock_app() is needed (P5-003 resolved). TEST-006:
    // asserts the scan's leads are actually persisted, not merely that the call
    // returns Ok — an Ok with zero rows would pass a bare is_ok() check while
    // silently dropping every lead.
    #[tokio::test]
    async fn test_daily_scan_persists_model_leads_and_marks_run_completed() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        struct FakeLlmClient;
        #[async_trait::async_trait]
        impl crate::core::llm::LlmClient for FakeLlmClient {
            async fn call(
                &self,
                _model: &str,
                _prompt: &str,
                _system: &str,
            ) -> Result<String, String> {
                Ok(r#"{"leads":[{"title":"Council overspend","summary":"Budget anomaly","original_url":"http://example.test/lead"}]}"#.to_string())
            }
        }
        let llm_client: Arc<dyn crate::core::llm::LlmClient> = Arc::new(FakeLlmClient);

        let res = crate::core::daily_scan::run_daily_scan(
            &db_conn,
            &llm_client,
            "aggregator prompt template",
            "Brighton",
            "CO",
            24,
        )
        .await;

        let run_id = res.expect("scan should succeed");

        let conn = db_conn.lock().unwrap();
        let (count, title): (i32, String) = conn
            .query_row(
                "SELECT COUNT(*), COALESCE(MAX(title), '') FROM daily_scan_leads WHERE scan_id = ?1",
                [run_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(count, 1, "the scan's single lead should be persisted");
        assert_eq!(
            title, "Council overspend",
            "the persisted lead should carry the model's title"
        );

        let status: String = conn
            .query_row(
                "SELECT run_status FROM daily_scan_runs WHERE id = ?1",
                [run_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            status, "completed",
            "a successful scan should mark the run completed"
        );
    }

    #[test]
    fn test_migration_0007_survives_existing_evidence_rows() {
        let conn = Connection::open_in_memory().unwrap();
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();

        // Run migrations up to 0006
        conn.execute_batch(include_str!("../../migrations/0001_init.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0003_settings.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0004_source_tier.sql"))
            .unwrap();
        conn.execute_batch(include_str!("../../migrations/0005_daily_scans.sql"))
            .unwrap();
        conn.execute_batch(include_str!(
            "../../migrations/0006_daily_scan_lead_source_nullable.sql"
        ))
        .unwrap();

        // Insert a source
        conn.execute(
            "INSERT INTO sources (id, name, url, type, tier) VALUES (1, 'Test Source', 'http://example.com', 'rss', 'community_signal')",
            [],
        )
        .unwrap();

        // Insert an evidence item referencing that source
        conn.execute(
            "INSERT INTO evidence_items (id, source_id, fetched_at, excerpt, content_hash, entities) VALUES (1, 1, '2026-05-26T17:39:19Z', 'excerpt', 'hash123', '[]')",
            [],
        )
        .unwrap();

        // Run migration 0007 and make sure it doesn't fail due to foreign key violations/constraints on DROP TABLE
        let migration_res =
            conn.execute_batch(include_str!("../../migrations/0007_source_tier_check.sql"));
        assert!(
            migration_res.is_ok(),
            "Migration 0007 failed under active foreign keys: {:?}",
            migration_res
        );

        // Verify data was preserved
        let source_count: i32 = conn
            .query_row("SELECT COUNT(*) FROM sources", [], |row| row.get(0))
            .unwrap();
        let evidence_count: i32 = conn
            .query_row("SELECT COUNT(*) FROM evidence_items", [], |row| row.get(0))
            .unwrap();
        assert_eq!(source_count, 1);
        assert_eq!(evidence_count, 1);
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: the core scan reads the model from settings itself, so
    // the fake client can assert it without a Tauri mock_app() (P5-003 resolved).
    #[tokio::test]
    async fn test_daily_scan_uses_settings_model_not_hardcoded() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        // Insert custom model setting
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('model.selected', 'my-custom-model')",
            [],
        )
        .unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        struct FakeLlmClient;
        #[async_trait::async_trait]
        impl crate::core::llm::LlmClient for FakeLlmClient {
            async fn call(
                &self,
                model: &str,
                _prompt: &str,
                _system: &str,
            ) -> Result<String, String> {
                assert_eq!(model, "my-custom-model");
                Ok("{\"leads\":[]}".to_string())
            }
        }
        let llm_client: Arc<dyn crate::core::llm::LlmClient> = Arc::new(FakeLlmClient);

        let res = crate::core::daily_scan::run_daily_scan(
            &db_conn,
            &llm_client,
            "aggregator prompt template",
            "Brighton",
            "CO",
            24,
        )
        .await;

        assert!(res.is_ok());
    }

    struct NoopPullSink;
    impl crate::core::llm::PullProgressSink for NoopPullSink {
        fn progress(&self, _payload: crate::core::llm::PullProgress) {}
        fn complete(&self) {}
        fn error(&self, _message: String) {}
    }

    // Polls `cond` until it holds or `timeout` elapses, returning the final
    // result. Replaces fixed `sleep`s when waiting for an async side effect
    // (e.g. a cancellation propagating through CANCEL_PULL_MAP): a fixed sleep
    // either flakes under load if too short or wastes wall-clock if too long.
    async fn poll_until<F: Fn() -> bool>(timeout: std::time::Duration, cond: F) -> bool {
        let start = tokio::time::Instant::now();
        while start.elapsed() < timeout {
            if cond() {
                return true;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        cond()
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: drives the core pull against a local stub server on an
    // OS-assigned ephemeral port with a no-op sink, so no Tauri mock_app() is
    // needed and parallel tests never collide on a fixed port (P5-003 resolved).
    #[tokio::test]
    async fn test_pull_ollama_model_propagates_http_error() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let app = axum::Router::new().route(
            "/api/pull",
            axum::routing::post(|| async { (axum::http::StatusCode::NOT_FOUND, "Not Found") }),
        );

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let res = crate::core::llm::run_ollama_pull(
            "non-existent-model-xyz".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("status 404"));
    }

    // MUTATION-RESISTANT (per Amendments 001/002). Runs on every platform,
    // including Windows: exercises the real per-model cancellation map without a
    // Tauri mock_app(); the stub server uses an ephemeral port (P5-003 resolved).
    // Cancelling one model's pull must not disturb another model's in-flight pull.
    #[tokio::test]
    async fn test_cancel_ollama_pull_is_per_model() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let app = axum::Router::new().route(
            "/api/pull",
            axum::routing::post(|| async {
                let stream = futures_util::stream::unfold(0, |state| async move {
                    if state < 5 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        Some((
                            Ok::<_, axum::Error>(bytes::Bytes::from(
                                "{\"status\":\"downloading\"}\n",
                            )),
                            state + 1,
                        ))
                    } else {
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        None
                    }
                });
                axum::response::Response::new(axum::body::Body::from_stream(stream))
            }),
        );

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let res1 = crate::core::llm::run_ollama_pull(
            "model-1".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(res1.is_ok());

        let res2 = crate::core::llm::run_ollama_pull(
            "model-2".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(res2.is_ok());

        {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            assert!(map.contains_key("model-1"));
            assert!(map.contains_key("model-2"));
        }

        crate::core::llm::cancel_pull("model-1");

        // Wait (bounded) for the cancellation to drain model-1 from the map.
        let model_1_removed = poll_until(std::time::Duration::from_secs(5), || {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            !map.contains_key("model-1")
        })
        .await;
        assert!(
            model_1_removed,
            "cancelling model-1 should remove its entry from CANCEL_PULL_MAP"
        );

        {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            assert!(
                map.contains_key("model-2"),
                "cancelling model-1 must not disturb model-2's in-flight pull"
            );
        }

        crate::core::llm::cancel_pull("model-2");
    }

    // ENG-001: a second pull for a model that already has one in flight must be
    // rejected, not silently overwrite the first's cancel sender (which would
    // orphan the first pull and let the first's completion remove the second's
    // entry). Verifies the duplicate is refused and the original entry survives.
    #[tokio::test]
    async fn test_duplicate_same_model_pull_is_rejected() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let app = axum::Router::new().route(
            "/api/pull",
            axum::routing::post(|| async {
                // Keep the connection open so the pull stays in flight.
                let stream = futures_util::stream::unfold(0, |state| async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    Some((
                        Ok::<_, axum::Error>(bytes::Bytes::from("{\"status\":\"downloading\"}\n")),
                        state + 1,
                    ))
                });
                axum::response::Response::new(axum::body::Body::from_stream(stream))
            }),
        );

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let res1 = crate::core::llm::run_ollama_pull(
            "dup-model".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(res1.is_ok(), "the first pull should start");

        let res2 = crate::core::llm::run_ollama_pull(
            "dup-model".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(
            res2.is_err(),
            "a duplicate same-model pull must be rejected, not started"
        );
        assert!(
            res2.unwrap_err().contains("already in progress"),
            "the rejection should explain a pull is already in progress"
        );

        // The original pull's entry must still be present (not clobbered).
        {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            assert!(
                map.contains_key("dup-model"),
                "the original in-flight pull's cancel sender must survive the rejected duplicate"
            );
        }

        crate::core::llm::cancel_pull("dup-model");
        let removed = poll_until(std::time::Duration::from_secs(5), || {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            !map.contains_key("dup-model")
        })
        .await;
        assert!(
            removed,
            "cancelling the original pull should drain its entry"
        );
    }

    // ENG-006: a base_url carrying a trailing slash must not produce a
    // double-slashed "//api/pull" (which the stub server — like real Ollama —
    // would not route, surfacing as an error). The stub only registers
    // "/api/pull", so an Ok here proves the trailing slash was trimmed.
    #[tokio::test]
    async fn test_run_ollama_pull_trims_trailing_slash_in_base_url() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let app = axum::Router::new().route(
            "/api/pull",
            axum::routing::post(|| async {
                (axum::http::StatusCode::OK, "{\"status\":\"success\"}\n")
            }),
        );

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        // Note the trailing slash on the base URL.
        let base_url = format!("http://{}/", addr);
        let res = crate::core::llm::run_ollama_pull(
            "slash-model".to_string(),
            &base_url,
            std::sync::Arc::new(NoopPullSink),
        )
        .await;
        assert!(
            res.is_ok(),
            "a trailing-slash base_url should still reach /api/pull, got {:?}",
            res
        );

        // The short body completes the stream, draining the entry on its own.
        let drained = poll_until(std::time::Duration::from_secs(5), || {
            let map = crate::core::llm::CANCEL_PULL_MAP.lock().unwrap();
            !map.contains_key("slash-model")
        })
        .await;
        assert!(
            drained,
            "the completed pull should remove its own map entry"
        );
    }

    // B-1 remediation: the collision-detection that drives start()'s skip path
    // is extracted into port_in_use(), so the cross-platform coexistence
    // guarantee is verified on every platform (including Windows) without
    // constructing a Tauri AppHandle. mock_app() is incompatible with Windows
    // console-mode lib unit tests, which is why the prior test was ignored there
    // and the cross-platform claim was unproven on Windows.
    #[tokio::test]
    async fn test_port_in_use_detects_listener_cross_platform() {
        // Bind an OS-assigned ephemeral port so this test is isolated from
        // whatever may be running on the real Ollama port (11434).
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap().to_string();

        assert!(
            crate::core::llm::OllamaSidecar::port_in_use(&addr),
            "a bound listener at {addr} must be detected as in use; this is the \
             collision check start() relies on to skip spawning the sidecar"
        );

        drop(listener);

        assert!(
            !crate::core::llm::OllamaSidecar::port_in_use(&addr),
            "once the listener is dropped, {addr} must no longer be reported in use"
        );
    }

    // The full start() skip path — returns Ok and spawns no child when the
    // probed port is already in use — verified cross-platform via start_for_test,
    // which injects the probe address so we neither bind the real 11434 (a
    // developer's actual ollama may hold it) nor construct an AppHandle.
    #[test]
    fn test_sidecar_skips_spawn_when_port_occupied() {
        // Hold an OS-assigned port for the duration of the test so the probe
        // sees it as occupied.
        let _occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = _occupied.local_addr().unwrap().to_string();

        let sidecar = crate::core::llm::OllamaSidecar::new();
        let res = sidecar.start_for_test(&addr);
        assert!(res.is_ok());

        // No child should have been spawned because the port was in use.
        let child_guard = sidecar.child.lock().unwrap();
        assert!(child_guard.is_none());
    }

    // TEST-002: the spawn side of start()'s control flow — when the probed port
    // is free, start_internal acquires the guard and spawns a child. Exercised
    // via start_for_test (a faithful mirror of start() post-ENG-004) so the loop
    // wiring, not just the port-check predicate, is covered without a Tauri
    // AppHandle. The bundled fixture runs until killed; OllamaSidecar's Drop (and
    // the explicit stop() here) reaps it so no process leaks.
    #[test]
    fn test_sidecar_spawns_when_port_free() {
        // Bind then immediately release to obtain a port number that is free for
        // the spawn attempt.
        let addr = {
            let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            l.local_addr().unwrap().to_string()
        };

        let sidecar = crate::core::llm::OllamaSidecar::new();
        let res = sidecar.start_for_test(&addr);
        assert!(res.is_ok(), "spawn path should succeed, got {:?}", res);

        {
            let child_guard = sidecar.child.lock().unwrap();
            assert!(
                child_guard.is_some(),
                "a child must be spawned when the probed port is free"
            );
        }

        // Reap the spawned fixture deterministically.
        let _ = sidecar.stop();
        let child_guard = sidecar.child.lock().unwrap();
        assert!(
            child_guard.is_none(),
            "stop() must clear the spawned child"
        );
    }

    // ENG-004: start() must never kill a process it did not spawn. The orphan
    // sweep that previously enumerated processes and force-killed anything
    // matching `ollama ... serve` has been removed; coexistence is delivered
    // solely by the port-collision early-return. This test pins that policy by
    // asserting an already-listening port causes start_for_test to return Ok
    // while spawning nothing (no enumeration, no kill).
    #[test]
    fn test_sidecar_does_not_kill_external_listener() {
        let _occupied = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = _occupied.local_addr().unwrap().to_string();

        let sidecar = crate::core::llm::OllamaSidecar::new();
        let res = sidecar.start_for_test(&addr);
        assert!(res.is_ok());

        // The external listener is untouched and still accepting connections.
        assert!(
            crate::core::llm::OllamaSidecar::port_in_use(&addr),
            "an external listener must survive start(): we never kill processes we did not spawn"
        );
        let child_guard = sidecar.child.lock().unwrap();
        assert!(child_guard.is_none());
    }

    // TEST-001: compute_hash is the sole dedup key for evidence. A silent change
    // to its output (e.g. normalizing whitespace/case, or a crate swap) would
    // re-ingest every item as "new" and flood the lead queue. Pin the algorithm
    // with a golden vector and assert the raw, no-normalization contract.
    #[test]
    fn test_compute_hash_is_stable_and_pinned() {
        use crate::core::scraper::compute_hash;

        // Golden: SHA-256("hello world") as lowercase hex. If this ever changes,
        // dedup semantics changed and every stored content_hash is invalidated.
        assert_eq!(
            compute_hash("hello world"),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );

        // Deterministic: same input always hashes the same.
        assert_eq!(compute_hash("Council agenda"), compute_hash("Council agenda"));

        // Distinct inputs hash differently.
        assert_ne!(compute_hash("a"), compute_hash("b"));

        // No normalization: case and surrounding whitespace are significant, so
        // these must NOT collide (otherwise trivially-different excerpts would be
        // treated as duplicates and dropped).
        assert_ne!(compute_hash("Hello"), compute_hash("hello"));
        assert_ne!(compute_hash("agenda"), compute_hash("agenda "));
        assert_ne!(compute_hash("agenda"), compute_hash(" agenda"));
    }

    // TEST-001: extract_entities feeds the Money-Threshold and Watchlist
    // detectors. Pin exactly which dollar amounts and formal-org names a fixture
    // excerpt yields, the empty-on-no-match case, and the sort+dedup contract.
    // Known quirk pinned on purpose: the org regex greedily absorbs a preceding
    // capitalized word, so a sentence-initial "The City Council" is captured with
    // the leading article, while a mid-sentence "the new Parks Department" (after
    // lowercase words) is captured clean. This fixture locks that exact behavior
    // so any change to the regex's leading-word handling is caught as a regression.
    #[test]
    fn test_extract_entities_fixture() {
        use crate::core::scraper::extract_entities;

        let excerpt = "The City Council approved $1,250,000 for the new Parks Department. \
                       The School Board also met. Total was $500. A refund of $500 was issued.";
        let entities = extract_entities(excerpt);

        // Sorted (ASCII: '$' < uppercase letters) and de-duplicated ($500 twice).
        assert_eq!(
            entities,
            vec![
                "$1,250,000".to_string(),
                "$500".to_string(),
                "Parks Department".to_string(),
                "The City Council".to_string(),
                "The School Board".to_string(),
            ]
        );

        // Negative case: prose with no dollar amounts and no capitalized
        // org-suffix phrase yields nothing (lowercase "council" must not match).
        assert!(extract_entities("the council met to discuss the budget at city hall.").is_empty());
    }
}
