use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct DbTag {
    pub id: i64,
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
