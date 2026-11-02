use super::*;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let test_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is before UNIX epoch")
        .as_nanos();

    let new_tag = ServiceTag {
        name: format!("test_tag_{test_id}"),
        color: Some("#000".to_string()),
    };

    let id = add(&pool, new_tag).await.expect("Inserting failed");
    assert!(id > 0, "L'ID restituito deve essere maggiore di 0");

    let edited_tag = ServiceTag {
        name: format!("edited_tag_{test_id}"),
        color: Some("#FFF".to_string()),
    };
    edit(&pool, id, edited_tag).await.expect("Edit failed");

    delete(&pool, id).await.expect("Deleting failed");
}
