use crate::application::api_model::area_api_model::*;
use crate::application::controller::request_guard::ValidatedUser;
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

#[put("/area/<area_id>/description", format = "json", data = "<req>")]
pub fn put_area_description(
    area_id: u32,
    req: Json<AreaDescriptionRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    area_repository::put_area_description(area_id, &req.description);
    Ok(())
}
