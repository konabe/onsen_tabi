use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{domain::hotel_entity::HotelEntity, infrastructure::hotel_repository};

use super::api_model::{HotelRequest, HotelResponse};

#[get("/hotel")]
pub fn get_hotels() -> Json<Vec<HotelResponse>> {
    let hotels = hotel_repository::get_hotels();
    let response = hotels
        .iter()
        .map(|v| HotelResponse::from(v.clone()))
        .collect();
    Json(response)
}

#[get("/hotel/<hotel_id>")]
pub fn get_hotel(hotel_id: u32) -> Result<Json<HotelResponse>, Status> {
    let hotel = hotel_repository::get_hotel(hotel_id);
    match &hotel {
        Some(hotel) => Ok(Json(HotelResponse::from(hotel.clone()))),
        None => Err(Status::NotFound),
    }
}

#[post("/hotel", format = "json", data = "<hotel_req>")]
pub fn post_hotel(hotel_req: Json<HotelRequest>) -> Result<Json<HotelResponse>, Status> {
    let hotel_entity = HotelEntity::new(0, &hotel_req.name, hotel_req.has_washitsu, &vec![]);
    if let Some(hotel_entity) = hotel_entity {
        let created_hotel = hotel_repository::post_hotel(hotel_entity);
        return Ok(Json(HotelResponse::from(created_hotel.clone())));
    } else {
        return Err(Status::InternalServerError);
    };
}
