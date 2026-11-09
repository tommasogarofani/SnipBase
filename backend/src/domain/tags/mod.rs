pub mod models;
use crate::service;
use models::DomainTag;
use sqlx::PgPool;

#[cfg(test)]
mod test;

pub async fn get_all(pool: &PgPool) -> Result<Vec<DomainTag>, sqlx::Error> {
    Ok(service::tags::get_all(pool)
        .await?
        .into_iter()
        .map(DomainTag::from)
        .collect())
}

pub async fn add(pool: &PgPool, new_tag: DomainTag) -> Result<i64, sqlx::Error> {
    Ok(service::tags::add(pool, new_tag.into()).await?)
}

pub async fn edit(pool: &PgPool, id: i64, edited_tag: DomainTag) -> Result<(), sqlx::Error> {
    service::tags::edit(pool, id, edited_tag.into()).await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    service::tags::delete(pool, id).await
}
