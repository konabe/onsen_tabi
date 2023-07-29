use rocket_dyn_templates::serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub has_washitsu: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelRequest {
    pub name: String,
    pub has_washitsu: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub sprint_quality: String,
    pub liquid: String,
    pub ostomic_pressure: String,
    pub form: String,
}
