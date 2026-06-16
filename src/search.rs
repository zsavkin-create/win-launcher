use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub score: f32,
}

#[tauri::command]
pub fn search_query(query: String) -> Vec<SearchResult> {
    let matcher = SkimMatcherV2::default();
    // TODO: load from index
    vec![
        SearchResult { id: "1".to_string(), title: "Google Chrome".to_string(), subtitle: Some("Browser".to_string()), score: 95.0 },
        SearchResult { id: "2".to_string(), title: "VS Code".to_string(), subtitle: Some("Editor".to_string()), score: 90.0 },
    ]
}