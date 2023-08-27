#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;
mod schema;

use application::controller::area_controller::*;
use application::controller::hotel_controller::*;
use application::controller::onsen_controller::*;
use application::controller::user_controller::*;
use rocket_cors::CorsOptions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                get_hotels,
                get_hotel,
                post_hotel,
                put_hotel_description,
                get_onsens,
                get_onsen,
                post_onsen,
                put_onsen,
                put_onsen_description,
                get_areas,
                get_area,
                put_area_description,
                post_signup,
                post_signin,
            ],
        )
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch();
}
