#[macro_use]
extern crate rocket;

use onsen_tabi::application::controller::area_controller::*;
use onsen_tabi::application::controller::hotel_controller::*;
use onsen_tabi::application::controller::onsen_controller::*;
use onsen_tabi::application::controller::user_controller::*;
use onsen_tabi::CORS;
use rocket::response;
use rocket::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct PfResponse;

impl<'r> response::Responder<'r, 'static> for PfResponse {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .header(rocket::http::Header::new(
                "Access-Control-Allow-Origin",
                "*",
            ))
            .header(rocket::http::Header::new(
                "Access-Control-Allow-Methods",
                "POST, PUT, PATCH, GET, DELETE",
            ))
            .header(rocket::http::Header::new(
                "Access-Control-Allow-Headers",
                "*",
            ))
            .header(rocket::http::Header::new(
                "Access-Control-Allow-Credentials",
                "true",
            ))
            .ok()
    }
}

#[options("/<_p..>")]
pub fn preflight(_p: std::path::PathBuf) -> PfResponse {
    PfResponse
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
                preflight,
            ],
        )
        .attach(CORS)
}
