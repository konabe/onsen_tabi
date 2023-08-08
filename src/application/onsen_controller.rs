use rocket::http::Status;
use rocket_contrib::json::Json;
use std::string::ToString;

use crate::infrastructure::onsen_repository;

use super::api_model::OnsenResponse;

#[get("/onsen")]
pub fn get_onsens() -> Json<Vec<OnsenResponse>> {
    let onsens = onsen_repository::get_onsens();
    let response = onsens
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

#[get("/onsen/<onsen_id>")]
pub fn get_onsen(onsen_id: u32) -> Result<Json<OnsenResponse>, Status> {
    let other_onsen = onsen_repository::get_onsen(onsen_id);
    match other_onsen {
        Some(some_hotel) => Ok(Json(OnsenResponse {
            id: some_hotel.id,
            name: some_hotel.name.clone(),
            sprint_quality: some_hotel.spring_quality.clone(),
            liquid: some_hotel.liquid.as_ref().map(|v| v.to_string()),
            ostomic_pressure: some_hotel.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: some_hotel.form.to_string(),
        })),
        None => Err(Status::NotFound),
    }
}
