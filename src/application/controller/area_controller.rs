use crate::application::api_model::{area_request::*, area_response::*};
use crate::application::controller::request_guard::ValidatedUser;
use crate::infrastructure::repository::area_repository;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/area")]
pub fn get_areas() -> Json<Vec<AreaResponse>> {
    let areas = area_repository::get_areas_with_onsen();
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

#[put("/area/<area_id>", format = "json", data = "<area_req>")]
pub fn put_area(
    area_id: u32,
    area_req: Json<AreaRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let area_entity = area_req.create_entity(area_id);
    if let Some(area_entity) = area_entity {
        let _ = area_repository::put_area(area_entity);
    } else {
        return Err(Status::BadRequest);
    }
    Ok(())
}

#[post("/area", format = "json", data = "<area_req>")]
pub fn post_area(
    area_req: Json<AreaRequest>,
    user: ValidatedUser,
) -> Result<Json<AreaResponse>, Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let area_entity = area_req.create_entity(0);
    if let Some(area_entity) = area_entity {
        let created_area = area_repository::post_area(area_entity);
        return Ok(Json(AreaResponse::from(created_area.clone())));
    } else {
        return Err(Status::BadRequest);
    }
}
