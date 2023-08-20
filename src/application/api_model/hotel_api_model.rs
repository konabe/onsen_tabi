use crate::application::api_model::onsen_api_model::OnsenResponse;
use crate::domain::hotel_entity::HotelEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelRequest {
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub onsens: Vec<OnsenResponse>,
}

impl From<HotelEntity> for HotelResponse {
    fn from(value: HotelEntity) -> Self {
        Self {
            id: value.id as i32,
            name: value.name.to_string(),
            has_washitsu: value.has_washitsu,
            url: value.url,
            onsens: value
                .onsens
                .iter()
                .map(|v| OnsenResponse::from(v.clone()))
                .collect(),
        }
    }
}
