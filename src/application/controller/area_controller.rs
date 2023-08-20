use crate::application::api_model::area_api_model::*;
use crate::infrastructure::repository::area_repository;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/area")]
pub fn get_areas() -> Json<Vec<AreaResponse>> {
    let areas = area_repository::get_areas();
    let response: Vec<AreaResponse> = areas
        .iter()
        .map(|v| AreaResponse::from(v.clone()))
        .collect();
    Json(response)
}

#[get("/area/<area_id>")]
pub fn get_area(area_id: u32) -> Result<Json<AreaResponse>, Status> {
    let area = area_repository::get_area(area_id);
    match &area {
        Some(area) => Ok(Json(AreaResponse::from(area.clone()))),
        None => Err(Status::NotFound),
    }
}
