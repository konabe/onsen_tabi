use rocket_contrib::json::Json;

use crate::infrastructure::area_repository;

use super::api_model::AreaResponse;

#[get("/area")]
pub fn get_areas() -> Json<Vec<AreaResponse>> {
    let areas = area_repository::get_areas();
    let response: Vec<AreaResponse> = areas
        .iter()
        .map(|v| AreaResponse::from(v.clone()))
        .collect();
    Json(response)
}
