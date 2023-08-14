use serde::{Deserialize, Serialize};

use crate::domain::hotel_entity::HotelEntity;

use super::onsen_api_model::OnsenResponse;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelRequest {
    pub name: String,
    pub has_washitsu: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub has_washitsu: bool,
    pub onsens: Vec<OnsenResponse>,
}

impl From<HotelEntity> for HotelResponse {
    fn from(value: HotelEntity) -> Self {
        Self {
            id: value.id as i32,
            name: value.name.to_string(),
            has_washitsu: value.has_washitsu,
            onsens: value
                .onsens
                .iter()
                .map(|v| OnsenResponse::from(v.clone()))
                .collect(),
        }
    }
}
