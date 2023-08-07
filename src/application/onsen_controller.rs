use rocket_contrib::json::Json;
use std::string::ToString;

use crate::infrastructure::onsen_repository;

use super::api_model::OnsenResponse;

#[get("/onsen")]
pub fn get_onsens() -> Json<Vec<OnsenResponse>> {
    let hotels = onsen_repository::get_onsens();
    let response = hotels
        .iter()
        .map(|r| OnsenResponse {
            id: r.id,
            name: r.name.clone(),
            sprint_quality: r.spring_quality.clone(),
            liquid: r.liquid.as_ref().map(|v| v.to_string()),
            ostomic_pressure: r.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: r.form.to_string(),
        })
        .collect();
    Json(response)
}
