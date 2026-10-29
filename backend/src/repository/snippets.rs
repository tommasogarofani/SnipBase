pub use chrono::{DateTime, Utc};
pub struct DbSnippet{
        pub id: i128,
        pub title: String,
        pub description: Option<String>,
        pub content: String,
        pub language: String,
        pub public: bool,
        pub created_at: DateTime<Utc>
}

pub struct RepoSnippet{
        pub title: String,
        pub description: Option<String>,
        pub content: String,
        pub language: String,
        pub public: bool,
}