#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;
mod schema;

use application::hotel_controller::{
    static_rocket_route_info_for_get_hotel, static_rocket_route_info_for_get_hotels,
    static_rocket_route_info_for_post_hotel,
};
use application::onsen_controller::{
    static_rocket_route_info_for_get_onsen, static_rocket_route_info_for_get_onsens,
};
use rocket_cors::CorsOptions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, get_hotels, get_hotel, post_hotel, get_onsens, get_onsen],
        )
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch();
}
