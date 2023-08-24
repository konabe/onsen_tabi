use crate::domain::area_entity::AreaEntity;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaResponse {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub url: String,
    pub description: String,
}

impl From<AreaEntity> for AreaResponse {
    fn from(value: AreaEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prefecture: value.prefecture,
            url: value.url,
            description: value.description,
        }
    }
}
