use crate::service::snippets::models::ServiceSnippet;
pub use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct DbSnippet {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub language: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug)]
pub struct RepoSnippet {
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub language: String,
    pub public: bool,
}

impl From<DbSnippet> for RepoSnippet {
    fn from(value: DbSnippet) -> Self {
        RepoSnippet {
            title: (value.title),
            description: (value.description),
            content: (value.content),
            language: (value.language),
            public: (value.public),
        }
    }
}

impl From<RepoSnippet> for ServiceSnippet {
    fn from(value: RepoSnippet) -> Self {
        Self {
            title: (value.title),
            description: (value.description),
            content: (value.content),
            language: (value.language),
            public: (value.public),
        }
    }
}

impl From<ServiceSnippet> for RepoSnippet {
    fn from(value: ServiceSnippet) -> Self {
        Self {
            title: (value.title),
            description: (value.description),
            content: (value.content),
            language: (value.language),
            public: (value.public),
        }
    }
}
