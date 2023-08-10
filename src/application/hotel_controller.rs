use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{domain::hotel_entity::HotelEntity, infrastructure::hotel_repository};

use super::api_model::{HotelRequest, HotelResponse, OnsenResponse};

#[get("/hotel")]
pub fn get_hotels() -> Json<Vec<HotelResponse>> {
    let hotels = hotel_repository::get_hotels();
    let response = hotels
        .iter()
        .map(|r| HotelResponse {
            id: r.id as i32,
            name: r.name.to_string(),
            has_washitsu: r.has_washitsu,
            onsens: r
                .onsens
                .iter()
                .map(|v| OnsenResponse {
                    id: v.id,
                    name: v.name.clone(),
                    sprint_quality: v.spring_quality.clone(),
                    liquid: v.liquid.as_ref().map(|v| v.to_string()),
                    ostomic_pressure: v.osmotic_pressure.as_ref().map(|v| v.to_string()),
                    form: v.form.to_string(),
                })
                .collect(),
        })
        .collect();
    Json(response)
}

#[get("/hotel/<hotel_id>")]
pub fn get_hotel(hotel_id: u32) -> Result<Json<HotelResponse>, Status> {
    let other_hotel = hotel_repository::get_hotel(hotel_id);
    match other_hotel {
        Some(other_hotel) => Ok(Json(HotelResponse {
            id: other_hotel.id as i32,
            name: other_hotel.name.to_string(),
            has_washitsu: other_hotel.has_washitsu,
            onsens: other_hotel
                .onsens
                .iter()
                .map(|v| OnsenResponse {
                    id: v.id,
                    name: v.name.clone(),
                    sprint_quality: v.spring_quality.clone(),
                    liquid: v.liquid.as_ref().map(|v| v.to_string()),
                    ostomic_pressure: v.osmotic_pressure.as_ref().map(|v| v.to_string()),
                    form: v.form.to_string(),
                })
                .collect(),
        })),
        None => Err(Status::NotFound),
    }
}

#[post("/hotel", format = "json", data = "<hotel_req>")]
pub fn post_hotel(hotel_req: Json<HotelRequest>) -> Result<Json<HotelResponse>, Status> {
    let optional_hotel_entity =
        HotelEntity::new(0, &hotel_req.name, hotel_req.has_washitsu, &vec![]);
    if let Some(hotel_entity) = optional_hotel_entity {
        let other_hotel = hotel_repository::post_hotel(hotel_entity);
        return Ok(Json(HotelResponse {
            id: -1,
            name: other_hotel.name.to_string(),
            has_washitsu: other_hotel.has_washitsu,
            onsens: vec![],
        }));
    } else {
        return Err(Status::InternalServerError);
    };
}
