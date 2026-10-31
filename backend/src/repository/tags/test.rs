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
async fn test_tag_crud_flow() {
    let pool = setup_test_pool().await;
    let test_id = std::process::id();

    let new_tag = RepoTag {
        name: format!("test_tag_{test_id}"),
        color: Some("#000".to_string()),
    };

    let id = add(&pool, new_tag).await.expect("Inserting failed");
    assert!(id > 0, "L'ID restituito deve essere maggiore di 0");

    let edited_tag = RepoTag {
        name: format!("edited_tag_{test_id}"),
        color: Some("#FFF".to_string()),
    };
    edit(&pool, id, edited_tag).await.expect("Edit failed");

    delete(&pool, id).await.expect("Deleting failed");
}
