mod models;
use models::{DbTag, RepoTag};
use sqlx::{PgPool, Result};
#[cfg(test)]
mod test;

/// get_all returns a vec of every tags in the DB
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

/// add create a new tag into the DB
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

/// edit updates the fields of an exiting tag
pub async fn update(pool: &PgPool, id: i64, edited_tag: RepoTag) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            UPDATE tags
            SET name = $1, color = $2
            WHERE id = $3
            "#,
    )
    .bind(edited_tag.name)
    .bind(edited_tag.color)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// delete function, removes a tag from DB
pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
                DELETE FROM tags
                WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
