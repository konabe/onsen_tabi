use super::request_guard::ValidatedUser;
use crate::application::api_model::hotel_api_model::*;
use crate::domain::hotel_entity::HotelEntity;
use crate::infrastructure::repository::hotel_repository;
use rocket::http::Status;
use rocket_contrib::json::Json;

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
    let hotel_entity = HotelEntity::new(
        hotel_id,
        &hotel_req.name,
        hotel_req.has_washitsu,
        &hotel_req.url,
        &hotel_req.description,
        &vec![],
    );
    if let Some(hotel_entity) = hotel_entity {
        hotel_repository::put_hotel(hotel_entity);
    } else {
        return Err(Status::BadRequest);
    }
    Ok(())
}

#[put("/hotel/<hotel_id>/description", format = "json", data = "<req>")]
pub fn put_hotel_description(
    hotel_id: u32,
    req: Json<HotelDescriptionRequest>,
    user: ValidatedUser,
) -> Result<(), Status> {
    if user.role != "admin" {
        return Err(Status::Forbidden);
    }
    hotel_repository::put_hotel_description(hotel_id, &req.description);
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
    let hotel_entity = HotelEntity::new(
        0,
        &hotel_req.name,
        hotel_req.has_washitsu,
        &hotel_req.url,
        &hotel_req.description,
        &vec![],
    );
    if let Some(hotel_entity) = hotel_entity {
        let created_hotel = hotel_repository::post_hotel(hotel_entity);
        return Ok(Json(HotelResponse::from(created_hotel.clone())));
    } else {
        return Err(Status::BadRequest);
    };
}
