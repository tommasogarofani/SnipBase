use sqlx::{FromRow, PgPool, Result};

#[derive(Debug, FromRow)]
pub struct DbTag {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug)]
pub struct RepoTag {
    pub name: String,
    pub color: Option<String>,
}

impl From<DbTag> for RepoTag {
    fn from(value: DbTag) -> Self {
        RepoTag {
            name: (value.name),
            color: (value.color),
        }
    }
}

impl RepoTag {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let db_tags: Vec<DbTag> = sqlx::query_as::<_, DbTag>(
            r#"
                        SELECT id, name, color
                        FROM tags
                        "#,
        )
        .fetch_all(pool)
        .await?;

        let tags: Vec<RepoTag> = db_tags.into_iter().map(RepoTag::from).collect();

        Ok(tags)
    }

    pub async fn add(pool: &PgPool, new_tag: RepoTag) -> Result<Self, sqlx::Error> {
        let db_tag = sqlx::query_as::<_, DbTag>(
            r#"
            INSERT INTO tags (name, color)
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
}
