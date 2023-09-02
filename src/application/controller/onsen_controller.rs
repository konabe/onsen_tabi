use super::request_guard::ValidatedUser;
use crate::application::api_model::onsen_api_model::*;
use crate::infrastructure::repository::onsen_repository;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/onsen?<area_id>&<hotel_id>")]
pub fn get_onsens(area_id: Option<String>, hotel_id: Option<String>) -> Json<Vec<OnsenResponse>> {
    let area_id: Option<u32> = area_id.and_then(|v| v.parse().ok());
    let hotel_id: Option<u32> = hotel_id.and_then(|v| v.parse().ok());
    let onsens = onsen_repository::get_onsens(area_id, hotel_id);
    let response = onsens
        .iter()
        .map(|v| OnsenResponse::from(v.clone()))
        .collect();
    Json(response)
}

#[get("/onsen/<onsen_id>")]
pub fn get_onsen(onsen_id: u32) -> Result<Json<OnsenResponse>, Status> {
    let onsen = onsen_repository::get_onsen(onsen_id);
    match onsen {
        Some(onsen) => Ok(Json(OnsenResponse::from(onsen.clone()))),
        None => Err(Status::NotFound),
    }
}

#[put("/onsen/<onsen_id>", format = "json", data = "<onsen_req>")]
pub fn put_onsen(
    onsen_id: u32,
    onsen_req: Json<OnsenRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let onsen_entity = onsen_req.create_entity(onsen_id);
    if let Some(onsen_entity) = onsen_entity {
        let _ = onsen_repository::put_onsen(onsen_entity);
    } else {
        return Err(Status::BadRequest);
    }
    Ok(())
}

#[put("/onsen/<onsen_id>/description", format = "json", data = "<req>")]
pub fn put_onsen_description(
    onsen_id: u32,
    req: Json<OnsenDescriptionRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    onsen_repository::put_onsen_description(onsen_id, &req.description);
    Ok(())
}

#[post("/onsen", format = "json", data = "<onsen_req>")]
pub fn post_onsen(
    onsen_req: Json<OnsenRequest>,
    user: ValidatedUser,
) -> Result<Json<OnsenResponse>, Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let onsen_entity = onsen_req.create_entity(0);
    if let Some(onsen_entity) = onsen_entity {
        let created_hotel = onsen_repository::post_onsen(onsen_entity);
        return Ok(Json(OnsenResponse::from(created_hotel.clone())));
    } else {
        return Err(Status::BadRequest);
    }
}
