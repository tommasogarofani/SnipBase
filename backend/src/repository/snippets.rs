pub use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Result};

#[derive(Debug, FromRow)]
pub struct DbSnippet {
    pub id: i32,
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

impl RepoSnippet {
    /// get_all requests every snippet stored in database
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let db_snippets: Vec<DbSnippet> = sqlx::query_as::<_, DbSnippet>(
            r#"
                        SELECT id, title, description, content, language, public, created_at
                        FROM snippets
                        "#,
        )
        .fetch_all(pool)
        .await?;

        let snippets: Vec<RepoSnippet> = db_snippets.into_iter().map(RepoSnippet::from).collect();

        Ok(snippets)
    }

    /// get_by_id requests a single snippet searching it by the id field
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Self, sqlx::Error> {
        let db_snippet: DbSnippet = sqlx::query_as::<_, DbSnippet>(
            r#"
                SELECT id, title, description, content, language, public, created_at
                FROM snippets
                WHERE snippets.id = $1
                "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(RepoSnippet::from(db_snippet))
    }

    /// add function adds a snippet in the db returning the id
    pub async fn add(pool: &PgPool, new_snippet: RepoSnippet) -> Result<i32, sqlx::Error> {
        let id = sqlx::query_scalar(
            r#"
            INSERT INTO snippets (title, description, content, language, public)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(new_snippet.title)
        .bind(new_snippet.description)
        .bind(new_snippet.content)
        .bind(new_snippet.language)
        .bind(new_snippet.public)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    /// edit function, modify an existing snippet
    pub async fn edit(pool: &PgPool, id: i32, edited: RepoSnippet) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE snippets
            SET title = $1, description = $2, content = $3, language = $4, public=$5
            WHERE id = $6
            "#,
        )
        .bind(edited.title)
        .bind(edited.description)
        .bind(edited.content)
        .bind(edited.language)
        .bind(edited.public)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE *
                FROM snippet
                WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
