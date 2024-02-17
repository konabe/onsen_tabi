use crate::application::api_model::onsen_api_model::OnsenResponse;
use crate::domain::hotel_entity::HotelEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelRequest {
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub description: String,
}

impl HotelRequest {
    pub fn create_entity(&self, id: u32) -> Option<HotelEntity> {
        HotelEntity::new(
            id,
            self.name.as_str(),
            self.has_washitsu,
            self.url.as_str(),
            self.description.as_str(),
            &vec![],
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub description: String,
    pub onsens: Vec<OnsenResponse>,
}

impl From<HotelEntity> for HotelResponse {
    fn from(value: HotelEntity) -> Self {
        Self {
            id: value.id as i32,
            name: value.name.to_string(),
            has_washitsu: value.has_washitsu,
            url: value.url,
            description: value.description,
            onsens: value
                .onsens
                .iter()
                .map(|v| OnsenResponse::from(v.clone()))
                .collect(),
        }
    }
}
