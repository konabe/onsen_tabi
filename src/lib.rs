#[macro_use]
extern crate rocket;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod schema;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

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
                "POST, PUT, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_fairing_info() {
        let cors = CORS;
        let info = cors.info();
        assert_eq!(info.name, "Add CORS headers to responses");
        // Kind doesn't implement PartialEq, so we just verify it returns Info
        // The actual CORS behavior is tested in E2E tests
    }

    #[test]
    fn test_cors_struct_exists() {
        // Verify the CORS struct can be instantiated
        let _cors = CORS;
    }
}
