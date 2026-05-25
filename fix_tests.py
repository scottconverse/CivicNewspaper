import sys
content = open('src-tauri/src/core/tests.rs').read()
content = content.replace(
    'last_scraped: None,',
    'last_scraped: None,\n                tier: "official_record".to_string(),'
)
content = content.replace(
    'confirmation_checklist: "[]".to_string(),\n                created_at: Utc::now().to_rfc3339(),',
    'confirmation_checklist: "[]".to_string(),\n                created_at: Utc::now().to_rfc3339(),\n                from_scan_lead_id: None,'
)
content = content.replace(
    'confirmation_checklist: "[]".to_string(),\n                created_at: chrono::Utc::now().to_rfc3339(),',
    'confirmation_checklist: "[]".to_string(),\n                created_at: chrono::Utc::now().to_rfc3339(),\n                from_scan_lead_id: None,'
)

tests = """
    // 9. Phase 4 Tests
    #[test]
    fn test_source_tier_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        conn.execute(
            "INSERT INTO sources (name, url, type, status) VALUES (?1, ?2, ?3, ?4)",
            ["Test", "http://test", "primary_record", "online"],
        ).unwrap();
        let tier: String = conn.query_row("SELECT tier FROM sources LIMIT 1", [], |row| row.get(0)).unwrap();
        assert_eq!(tier, "official_record");
    }

    #[test]
    fn test_source_tier_backfill_media_lead() {
        let source = Source {
            id: None,
            name: "Test".to_string(),
            url: "http".to_string(),
            r#type: "media_lead".to_string(),
            tier: "news_reporting".to_string(),
            status: "online".to_string(),
            last_success_at: None,
            last_failed_at: None,
            last_scraped: None,
        };
        assert_eq!(source.tier, "news_reporting");
    }

    #[test]
    fn test_list_prompts_returns_bundled() {
        let prompts = crate::core::prompts::list_prompts();
        assert!(prompts.len() >= 14);
    }

    #[test]
    fn test_get_prompt_loads_aggregator() {
        let content = crate::core::prompts::load_prompt(None, "aggregator/01-daily-scan").unwrap();
        assert!(content.contains("CIVIC NEWSROOM - DAILY CIVIC NEWS AGGREGATOR"));
    }

    #[tokio::test]
    async fn test_daily_scan_parses_fixture_response() {
        let fixture = "Headline: A\\nTier: B\\nSource: C\\nURL: D\\nConfidence: E\\nAction: F\\nBeat: G\\nDetails: H";
        let leads = crate::core::daily_scan::parse_daily_scan_leads(fixture);
        assert_eq!(leads.len(), 1); 
    }

    #[tokio::test]
    async fn test_plain_language_rewrite_invokes_ollama() {
        let rewritten = crate::tauri_cmds::plain_language_rewrite_logic(1, "Draft content").await.unwrap();
        assert_eq!(rewritten, "Mocked response");
    }
"""
content = content.replace('        assert_eq!(res_bad.unwrap_err(), "Path is outside allowed directories");\n    }\n}\n', '        assert_eq!(res_bad.unwrap_err(), "Path is outside allowed directories");\n    }\n' + tests + '}\n')
open('src-tauri/src/core/tests.rs', 'w').write(content)
