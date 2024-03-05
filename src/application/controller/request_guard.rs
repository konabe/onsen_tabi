use crate::application::auth::jwt::decode_jwt;
use crate::infrastructure::repository::user_repository;
use chrono::{TimeZone, Utc};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

pub struct ValidatedUser {
    pub email: String,
    pub role: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ValidatedUser {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let bearer_token = request.headers().get_one("Authorization");

        if let Some(bearer_token) = bearer_token {
            let prefix = &bearer_token[..6];
            if prefix != "Bearer" {
                return Outcome::Error((Status::Unauthorized, ApiTokenError::Missing));
            }
            let token = &bearer_token[7..];
            if let Some(claims) = decode_jwt(token) {
                let email = claims.email;
                let exp = Utc.timestamp_opt(claims.exp as i64, 0).unwrap();
                if exp < Utc::now() {
                    return Outcome::Error((Status::Unauthorized, ApiTokenError::Missing));
                }

                let user = user_repository::get_user(&email);
                if let Some(user) = user {
                    return Outcome::Success(ValidatedUser {
                        email: user.email,
                        role: user.role,
                    });
                }
            }
        }
        return Outcome::Error((Status::Unauthorized, ApiTokenError::Missing));
    }
}
