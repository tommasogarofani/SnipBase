pub use chrono::{DateTime, Utc};
use sqlx::{FromRow, Result, PgPool};

#[derive(Debug, FromRow)]
pub struct DbSnippet{
        pub id: i32,
        pub title: String,
        pub description: Option<String>,
        pub content: String,
        pub language: String,
        pub public: bool,
        pub created_at: DateTime<Utc>
}
#[derive(Debug)]
pub struct RepoSnippet{
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
                        public: (value.public)
                }
        }
}

impl RepoSnippet {

        pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
                let db_snippets: Vec<DbSnippet> = sqlx::query_as::<_, DbSnippet>(
                         r#"
                        SELECT id, title, description, content, language, public, created_at
                        FROM snippets
                        "#,)
                        .fetch_all(pool)
                        .await?;

                let snippets: Vec<RepoSnippet> = db_snippets
                .into_iter()
                .map(RepoSnippet::from)
                .collect();

                Ok(snippets)
        }

pub async fn add(pool: &PgPool, new_snippet: RepoSnippet) -> Result<Self, sqlx::Error> {
        let db_snippet = sqlx::query_as::<_, DbSnippet>(
            r#"
            INSERT INTO snippets (title, description, content, language, public)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, content, language, public, created_at
            "#,
        )
        .bind(new_snippet.title)
        .bind(new_snippet.description)
        .bind(new_snippet.content)
        .bind(new_snippet.language)
        .bind(new_snippet.public)
        .fetch_one(pool)
        .await?;

        Ok(RepoSnippet::from(db_snippet))
    }
}

