pub mod models;
use crate::repository;
use models::ServiceSnippet;
use sqlx::PgPool;
#[cfg(test)]
mod test;

/// Retrieves all snippets from the database.
pub async fn get_all(pool: &PgPool) -> Result<Vec<ServiceSnippet>, sqlx::Error> {
    let snippets = repository::snippets::get_all(pool)
        .await?
        .into_iter()
        .map(ServiceSnippet::from)
        .collect();
    Ok(snippets)
}

/// Retrieves a snippet by its unique identifier.
pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<ServiceSnippet, sqlx::Error> {
    let snippet = ServiceSnippet::from(repository::snippets::get_by_id(pool, id).await?);
    Ok(snippet)
}

/// Adds a new snippet and returns its generated identifier.
pub async fn add(pool: &PgPool, new_snippet: ServiceSnippet) -> Result<i64, sqlx::Error> {
    let id = repository::snippets::add(pool, new_snippet.into()).await?;
    Ok(id)
}

/// Updates an existing snippet by its identifier.
pub async fn edit(pool: &PgPool, id: i64, edited: ServiceSnippet) -> Result<(), sqlx::Error> {
    repository::snippets::edit(pool, id, edited.into()).await?;
    Ok(())
}

/// Deletes a snippet by its identifier.
pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    repository::snippets::delete(pool, id).await?;
    Ok(())
}
