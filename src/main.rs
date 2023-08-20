#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod application;
mod domain;
mod infrastructure;
mod schema;

use application::controller::area_controller::{
    static_rocket_route_info_for_get_area, static_rocket_route_info_for_get_areas,
};
use application::controller::hotel_controller::{
    static_rocket_route_info_for_get_hotel, static_rocket_route_info_for_get_hotels,
    static_rocket_route_info_for_post_hotel,
};
use application::controller::onsen_controller::{
    static_rocket_route_info_for_get_onsen, static_rocket_route_info_for_get_onsens,
    static_rocket_route_info_for_post_onsen, static_rocket_route_info_for_put_onsen_description,
};
use application::controller::user_controller::{
    static_rocket_route_info_for_post_signin, static_rocket_route_info_for_post_signup,
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
            routes![
                index,
                get_hotels,
                get_hotel,
                post_hotel,
                get_onsens,
                get_onsen,
                post_onsen,
                get_areas,
                get_area,
                put_onsen_description,
                post_signup,
                post_signin,
            ],
        )
        .attach(CorsOptions::default().to_cors().expect("error"))
        .launch();
}
