use super::*;
use sqlx::PgPool;

async fn setup_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://tommaso:tommaso@localhost:5439/snipbase".to_string());

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
    let inserted_id = add(&pool, new_snippet).await;
    assert!(inserted_id.is_ok(), "Inserting failed");
    let id = inserted_id.unwrap();
    assert!(id > 0, "L'ID restituito deve essere maggiore di 0");

    // 3. get_by_id testing
    let fetched = get_by_id(&pool, id).await;
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
    let edited = edit(&pool, id, edited_snippet).await;
    assert!(edited.is_ok(), "Edit failed");

    // 4. delete testing
    let deleted = delete(&pool, id).await;
    assert!(deleted.is_ok(), "Deleting failed");
}
