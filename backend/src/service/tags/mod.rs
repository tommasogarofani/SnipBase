pub mod models;
use crate::repository;
use models::ServiceTag;
use sqlx::PgPool;
#[cfg(test)]
mod test;

/// get_all returns a vec of every tags in the DB
pub async fn get_all(pool: &PgPool) -> Result<Vec<ServiceTag>, sqlx::Error> {
    let tags = repository::tags::get_all(pool)
        .await?
        .into_iter()
        .map(ServiceTag::from)
        .collect();
    Ok(tags)
}

/// add create a new tag into the DB
pub async fn add(pool: &PgPool, new_tag: ServiceTag) -> Result<i64, sqlx::Error> {
    let id = repository::tags::add(pool, new_tag.into()).await?;
    Ok(id)
}

/// edit updates the fields of an exiting tag
pub async fn edit(pool: &PgPool, id: i64, edited_tag: ServiceTag) -> Result<(), sqlx::Error> {
    repository::tags::edit(pool, id, edited_tag.into()).await?;
    Ok(())
}

/// delete function, removes a tag from DB
pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    repository::tags::delete(pool, id).await?;
    Ok(())
}
