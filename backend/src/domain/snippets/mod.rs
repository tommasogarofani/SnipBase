pub mod models;
use crate::service;
use models::DomainSnippet;
use sqlx::PgPool;

#[cfg(test)]
mod test;

pub async fn get_all(pool: &PgPool) -> Result<Vec<DomainSnippet>, sqlx::Error> {
    Ok(service::snippets::get_all(pool)
        .await?
        .into_iter()
        .map(DomainSnippet::from)
        .collect())
}

pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<DomainSnippet, sqlx::Error> {
    Ok(DomainSnippet::from(
        service::snippets::get_by_id(pool, id).await?,
    ))
}

pub async fn add(pool: &PgPool, new_snippet: DomainSnippet) -> Result<i64, sqlx::Error> {
    Ok(service::snippets::add(pool, new_snippet.into()).await?)
}

pub async fn edit(pool: &PgPool, id: i64, edited: DomainSnippet) -> Result<(), sqlx::Error> {
    service::snippets::edit(pool, id, edited.into()).await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    service::snippets::delete(pool, id).await
}
