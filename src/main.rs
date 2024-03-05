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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_hotels,
                get_hotel,
                post_hotel,
                put_hotel,
                get_onsens,
                get_onsen,
                post_onsen,
                put_onsen,
                get_areas,
                get_area,
                post_area,
                put_area,
                post_signup,
                post_signin,
            ],
        )
        .attach(
            CorsOptions::default()
                .to_cors()
                .expect("Cors options error"),
        )
}
