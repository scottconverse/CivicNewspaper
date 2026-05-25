// core/prompts.rs

const VALID_PROMPT_IDS: &[&str] = &["aggregator"];

pub fn list_prompts() -> Vec<String> {
    VALID_PROMPT_IDS.iter().map(|s| s.to_string()).collect()
}

pub fn get_prompt(app: &tauri::AppHandle, id: &str) -> Result<String, String> {
    use tauri::Manager;
    if !VALID_PROMPT_IDS.contains(&id) {
        return Err(format!("Invalid prompt ID: {}", id));
    }
    
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;
    let prompt_path = resource_dir.join("prompts").join(format!("{}.md", id));
    
    std::fs::read_to_string(prompt_path).map_err(|e| e.to_string())
}
