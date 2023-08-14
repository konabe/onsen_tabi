use rocket_dyn_templates::serde;
use serde::{Deserialize, Serialize};

use crate::domain::{hotel_entity::HotelEntity, onsen_entity::OnsenEntity};

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenDescriptionRequest {
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub sprint_quality: String,
    pub liquid: Option<String>,
    pub ostomic_pressure: Option<String>,
    pub form: String,
    pub description: String,
}

impl From<OnsenEntity> for OnsenResponse {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            sprint_quality: value.spring_quality.clone(),
            liquid: value.liquid.as_ref().map(|v| v.to_string()),
            ostomic_pressure: value.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: value.form.to_string(),
            description: value.description.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaResponse {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
}
