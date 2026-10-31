pub use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Result};

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

    /// get_all requests every snippet stored in database
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let db_snippets: Vec<DbSnippet> = sqlx::query_as::<_, DbSnippet>(
            r#"
                        SELECT id, title, description, content, language, public, created_at
                        FROM snippet
                        "#,
        )
        .fetch_all(pool)
        .await?;

        let snippets: Vec<RepoSnippet> = db_snippets.into_iter().map(RepoSnippet::from).collect();

        Ok(snippets)
    }

    /// get_by_id requests a single snippet searching it by the id field
    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, sqlx::Error> {
        let db_snippet: DbSnippet = sqlx::query_as::<_, DbSnippet>(
            r#"
                SELECT id, title, description, content, language, public, created_at
                FROM snippet
                WHERE id = $1
                "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(RepoSnippet::from(db_snippet))
    }

    /// add function adds a snippet in the db returning the id
    pub async fn add(pool: &PgPool, new_snippet: RepoSnippet) -> Result<i64, sqlx::Error> {
        let id = sqlx::query_scalar(
            r#"
            INSERT INTO snippet (title, description, content, language, public)
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
    pub async fn edit(pool: &PgPool, id: i64, edited: RepoSnippet) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE snippet
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

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                DELETE FROM snippet
                WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }


#[cfg(test)]
mod tests {
    use super::*;
    async fn setup_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://test:test@localhost:5439/snipbase_test".to_string());

        PgPool::connect(&database_url)
            .await
            .expect("Unable to connect to DB")
    }

    #[tokio::test]
    async fn test_snippet_crud_flow() {
        let pool = setup_test_pool().await;

        // 1. creating a new test snippet
        let new_snippet = RepoSnippet {
            title: "Test Snippet".to_string(),
            description: Some("Test description".to_string()),
            content: "println!(\"Hello Test\");".to_string(),
            language: "rust".to_string(),
            public: true,
        };

        // 2. Add testing
        let inserted_id = RepoSnippet::add(&pool, new_snippet).await;
        assert!(inserted_id.is_ok(), "Inserting failed");
        let id = inserted_id.unwrap();
        assert!(id > 0, "L'ID restituito deve essere maggiore di 0");

        // 3. get_by_id testing
        let fetched = RepoSnippet::get_by_id(&pool, id).await;
        assert!(fetched.is_ok(), "Getting by id failed");
        let snippet = fetched.unwrap();
        assert_eq!(snippet.title, "Test Snippet");

        //4. edit testing
        let edited_snippet = RepoSnippet {
            title: "Test Snippet".to_string(),
            description: Some("Edited test description".to_string()),
            content: "println!(\"Hello Test\");".to_string(),
            language: "rust".to_string(),
            public: false,
        };
        let edited = RepoSnippet::edit(&pool, id, edited_snippet).await;
        assert!(edited.is_ok(), "Edit failed");

        // 4. delete testing
        let deleted = RepoSnippet::delete(&pool, id).await;
        assert!(deleted.is_ok(), "Deleting failed");
    }
}
