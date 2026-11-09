pub use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ServiceSnippet {
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub language: String,
    pub public: bool,
}

impl From<crate::domain::snippets::models::DomainSnippet> for ServiceSnippet {
    fn from(value: crate::domain::snippets::models::DomainSnippet) -> Self {
        Self {
            title: value.title,
            description: value.description,
            content: value.content,
            language: value.language,
            public: value.public,
        }
    }
}

impl From<ServiceSnippet> for crate::domain::snippets::models::DomainSnippet {
    fn from(value: ServiceSnippet) -> Self {
        Self {
            title: value.title,
            description: value.description,
            content: value.content,
            language: value.language,
            public: value.public,
        }
    }
}
