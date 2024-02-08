use crate::domain::area_entity::AreaEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaRequest {
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
}

impl AreaRequest {
    pub fn create_entity(&self, id: u32) -> Option<AreaEntity> {
        AreaEntity::new(
            id,
            self.name.as_str(),
            self.prefecture.as_str(),
            self.national_resort,
            self.village.as_deref(),
            self.url.as_str(),
            self.description.as_str(),
            vec![],
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaDescriptionRequest {
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaResponse {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
}

impl From<AreaEntity> for AreaResponse {
    fn from(value: AreaEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prefecture: value.prefecture,
            national_resort: value.national_resort,
            village: value.village,
            url: value.url,
            description: value.description,
        }
    }
}
