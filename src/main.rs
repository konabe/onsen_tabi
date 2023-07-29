#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod domain;
pub mod infrastructure;
mod schema;

use self::schema::hotel::dsl::*;
use crate::infrastructure::{diesel_connection::establish_connection, diesel_models::Hotel};
use diesel::prelude::*;
use infrastructure::api_response::HotelResponse;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hotel")]
fn get_hotels() -> Json<Vec<HotelResponse>> {
    let connection = &mut establish_connection();
    let results = hotel
        .select(Hotel::as_select())
        .load(connection)
        .expect("error");
    let response = results
        .iter()
        .map(|r| HotelResponse {
            id: r.id,
            name: r.name.to_string(),
            has_washitsu: r.has_washitsu,
        })
        .collect();
    Json(response)
}

#[get("/hotel/<hotel_id>")]
fn get_hotel(hotel_id: u32) -> Result<Json<HotelResponse>, Status> {
    let connection = &mut establish_connection();
    let results = hotel
        .select(Hotel::as_select())
        .load(connection)
        .expect("error");
    let result = results.iter().find(|r| r.id == hotel_id);
    match result {
        Some(other_hotel) => Ok(Json(HotelResponse {
            id: other_hotel.id,
            name: other_hotel.name.to_string(),
            has_washitsu: other_hotel.has_washitsu,
        })),
        None => Err(Status::NotFound),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_hotels, get_hotel])
        .launch();
}
