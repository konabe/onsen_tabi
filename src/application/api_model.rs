use rocket_dyn_templates::serde;
use serde::{Deserialize, Serialize};

use crate::domain::onsen_entity::OnsenEntity;

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

impl OnsenResponse {
    pub fn create(onsen: &OnsenEntity) -> Self {
        Self {
            id: onsen.id,
            name: onsen.name.clone(),
            sprint_quality: onsen.spring_quality.clone(),
            liquid: onsen.liquid.as_ref().map(|v| v.to_string()),
            ostomic_pressure: onsen.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: onsen.form.to_string(),
            description: onsen.description.to_string(),
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
