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
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response;
use rocket::Request;
use rocket::Response;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct PfResponse;

impl<'r> response::Responder<'r, 'static> for PfResponse {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'static> {
        response::Response::build()
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .header(Header::new(
                "Access-Control-Allow-Methods",
                "POST,GET,PATCH,OPTIONS,TRACE",
            ))
            .header(Header::new("Access-Control-Allow-Headers", "x-api-key"))
            .header(Header::new("Access-Control-Allow-Credentials", "true"))
            .ok()
    }
}

#[options("/<_p..>")]
pub fn preflight(_p: std::path::PathBuf) -> PfResponse {
    PfResponse
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
