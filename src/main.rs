#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod domain;
pub mod infrastructure;
mod schema;

use domain::hotel_entity::HotelEntity;
use infrastructure::{
    api_model::{HotelRequest, HotelResponse},
    hotel_repository,
};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hotel")]
fn get_hotels() -> Json<Vec<HotelResponse>> {
    let hotels = hotel_repository::get_hotels();
    let response = hotels
        .iter()
        .map(|r| HotelResponse {
            id: r.id as i32,
            name: r.name.to_string(),
            has_washitsu: r.has_washitsu,
        })
        .collect();
    Json(response)
}

#[get("/hotel/<hotel_id>")]
fn get_hotel(hotel_id: u32) -> Result<Json<HotelResponse>, Status> {
    let other_hotel = hotel_repository::get_hotel(hotel_id);
    match other_hotel {
        Some(other_hotel) => Ok(Json(HotelResponse {
            id: other_hotel.id as i32,
            name: other_hotel.name.to_string(),
            has_washitsu: other_hotel.has_washitsu,
        })),
        None => Err(Status::NotFound),
    }
}

#[post("/hotel", format = "json", data = "<hotel_req>")]
fn post_hotel(hotel_req: Json<HotelRequest>) -> Result<Json<HotelResponse>, Status> {
    let optional_hotel_entity = HotelEntity::new(0, &hotel_req.name, hotel_req.has_washitsu);
    if let Some(hotel_entity) = optional_hotel_entity {
        let other_hotel = hotel_repository::post_hotel(hotel_entity);
        return Ok(Json(HotelResponse {
            id: -1,
            name: other_hotel.name.to_string(),
            has_washitsu: other_hotel.has_washitsu,
        }));
    } else {
        return Err(Status::InternalServerError);
    };
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_hotels, get_hotel, post_hotel])
        .launch();
}
