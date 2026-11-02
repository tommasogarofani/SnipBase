pub use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ServiceSnippet {
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub language: String,
    pub public: bool,
}
