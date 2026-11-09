#[derive(Debug)]
pub struct ServiceTag {
    pub name: String,
    pub color: Option<String>,
}

impl From<crate::domain::tags::models::DomainTag> for ServiceTag {
    fn from(value: crate::domain::tags::models::DomainTag) -> Self {
        Self {
            name: value.name,
            color: value.color,
        }
    }
}

impl From<ServiceTag> for crate::domain::tags::models::DomainTag {
    fn from(value: ServiceTag) -> Self {
        Self {
            name: value.name,
            color: value.color,
        }
    }
}
