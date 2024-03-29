use super::request_guard::ValidatedUser;
use crate::application::api_model::hotel_request::*;
use crate::application::api_model::hotel_response::*;
use crate::infrastructure::repository::hotel_repository;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/hotel?<area_id>")]
pub fn get_hotels(area_id: Option<String>) -> Json<Vec<HotelResponse>> {
    let area_id: Option<u32> = area_id.and_then(|v| v.parse().ok());
    let hotels = hotel_repository::get_hotels(area_id);
    let response = hotels
        .iter()
        .map(|v| HotelResponse::from(v.clone()))
        .collect();
    Json(response)
}

#[get("/hotel/<hotel_id>")]
pub fn get_hotel(hotel_id: u32) -> Result<Json<HotelResponse>, Status> {
    let hotel = hotel_repository::get_hotel_with_onsen(hotel_id);
    match &hotel {
        Some(hotel) => Ok(Json(HotelResponse::from(hotel.clone()))),
        None => Err(Status::NotFound),
    }
}

#[put("/hotel/<hotel_id>", format = "json", data = "<hotel_req>")]
pub fn put_hotel(
    hotel_id: u32,
    hotel_req: Json<HotelRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let hotel_entity = hotel_req.create_entity(hotel_id);
    if let Some(hotel_entity) = hotel_entity {
        hotel_repository::put_hotel(hotel_entity);
    } else {
        return Err(Status::BadRequest);
    }
    Ok(())
}

#[post("/hotel", format = "json", data = "<hotel_req>")]
pub fn post_hotel(
    hotel_req: Json<HotelRequest>,
    user: ValidatedUser,
) -> Result<Json<HotelResponse>, Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    let hotel_entity = hotel_req.create_entity(0);
    if let Some(hotel_entity) = hotel_entity {
        let created_hotel = hotel_repository::post_hotel(hotel_entity);
        return Ok(Json(HotelResponse::from(created_hotel.clone())));
    } else {
        return Err(Status::BadRequest);
    };
}
