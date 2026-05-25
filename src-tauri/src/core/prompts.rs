// core/prompts.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMeta {
    pub id: String,
    pub category: String,
    pub title: String,
    pub path: String,
    pub description: String,
}

pub fn list_prompts() -> Vec<PromptMeta> {
    let mut prompts = Vec::new();
    
    // Aggregator
    prompts.push(PromptMeta {
        id: "aggregator/01-daily-scan".to_string(),
        category: "aggregator".to_string(),
        title: "Daily Scan Aggregator".to_string(),
        path: "aggregator/01-daily-scan.md".to_string(),
        description: "Aggregates evidence into daily scan leads".to_string(),
    });
    for i in 1..=3 {
        prompts.push(PromptMeta {
            id: format!("aggregator/agg_{}", i),
            category: "aggregator".to_string(),
            title: format!("Agg {}", i),
            path: format!("aggregator/agg_{}.md", i),
            description: "Mock".to_string(),
        });
    }

    // Story
    prompts.push(PromptMeta {
        id: "story/07-plain-language".to_string(),
        category: "story".to_string(),
        title: "Plain Language Rewrite".to_string(),
        path: "story/07-plain-language.md".to_string(),
        description: "Rewrites draft".to_string(),
    });
    for i in 1..=3 {
        prompts.push(PromptMeta {
            id: format!("story/story_{}", i),
            category: "story".to_string(),
            title: format!("Story {}", i),
            path: format!("story/story_{}.md", i),
            description: "Mock".to_string(),
        });
    }

    // Audit, Utility, Legal
    for cat in ["audit", "utility", "legal"].iter() {
        for i in 1..=3 {
            prompts.push(PromptMeta {
                id: format!("{}/{}_{}", cat, &cat[..4], i),
                category: cat.to_string(),
                title: format!("{} {}", cat, i),
                path: format!("{}/{}_{}.md", cat, &cat[..4], i),
                description: "Mock".to_string(),
            });
        }
    }
    
    prompts
}

pub fn load_prompt(app: Option<&tauri::AppHandle>, name: &str) -> Result<String, String> {
    let path = std::path::PathBuf::from(format!("prompts/{}.md", name));
    std::fs::read_to_string(path).map_err(|e| format!("Failed to read prompt: {}", e))
}
