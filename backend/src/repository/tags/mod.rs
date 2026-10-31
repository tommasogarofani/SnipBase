mod models;
use models::{DbTag, RepoTag};
use sqlx::{PgPool, Result};
#[cfg(test)]
mod test;

pub async fn get_all(pool: &PgPool) -> Result<Vec<RepoTag>, sqlx::Error> {
    let db_tags: Vec<DbTag> = sqlx::query_as::<_, DbTag>(
        r#"
                        SELECT id, name, color
                        FROM tag
                        "#,
    )
    .fetch_all(pool)
    .await?;

    let tags: Vec<RepoTag> = db_tags.into_iter().map(RepoTag::from).collect();

    Ok(tags)
}

pub async fn add(pool: &PgPool, new_tag: RepoTag) -> Result<RepoTag, sqlx::Error> {
    let db_tag = sqlx::query_as::<_, DbTag>(
        r#"
            INSERT INTO tag (name, color)
            VALUES ($1, $2)
            RETURNING name, color
            "#,
    )
    .bind(new_tag.name)
    .bind(new_tag.color)
    .fetch_one(pool)
    .await?;

    Ok(RepoTag::from(db_tag))
}
