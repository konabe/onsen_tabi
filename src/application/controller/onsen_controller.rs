use super::request_guard::ValidatedUser;
use crate::application::api_model::onsen_request::OnsenRequest;
use crate::application::api_model::onsen_response::*;
use crate::infrastructure::repository::{area_repository, onsen_repository};
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/onsen?<area_id>&<hotel_id>")]
pub fn get_onsens(area_id: Option<String>, hotel_id: Option<String>) -> Json<Vec<OnsenResponse>> {
    let area_id: Option<u32> = area_id.and_then(|v| v.parse().ok());
    let hotel_id: Option<u32> = hotel_id.and_then(|v| v.parse().ok());
    let onsens = onsen_repository::get_onsens(area_id, hotel_id);
    let response = onsens
        .iter()
        .map(|v| OnsenResponse::create(v.clone(), None))
        .collect();
    Json(response)
}

#[get("/onsen/<onsen_id>")]
pub fn get_onsen(onsen_id: u32) -> Result<Json<OnsenResponse>, Status> {
    let onsen = onsen_repository::get_onsen(onsen_id);
    match onsen {
        Some(onsen) => match onsen.area_id {
            Some(area_id) => {
                let area = area_repository::get_area(area_id);
                Ok(Json(OnsenResponse::create(onsen, area)))
            }
            None => Ok(Json(OnsenResponse::create(onsen, None))),
        },
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
        let created_onsen = onsen_repository::post_onsen(onsen_entity);
        return Ok(Json(OnsenResponse::create(created_onsen.clone(), None)));
    } else {
        return Err(Status::BadRequest);
    }
}
