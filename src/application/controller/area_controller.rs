use rocket_contrib::json::Json;

use crate::infrastructure::area_repository;

use crate::application::api_model::area_api_model::*;

#[get("/area")]
pub fn get_areas() -> Json<Vec<AreaResponse>> {
    let areas = area_repository::get_areas();
    let response: Vec<AreaResponse> = areas
        .iter()
        .map(|v| AreaResponse::from(v.clone()))
        .collect();
    Json(response)
}
