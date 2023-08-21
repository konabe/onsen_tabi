use std::env;

use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::infrastructure::repository::user_repository;

use super::user_controller::Claims;

pub struct ValidatedUser {
    pub email: String,
    pub role: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for ValidatedUser {
    type Error = ApiTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        dotenv().ok();
        let bearer_token = request.headers().get_one("Authorization");

        if let Some(bearer_token) = bearer_token {
            let token = &bearer_token[7..];
            let claims = decode::<Claims>(
                token,
                &DecodingKey::from_secret(
                    env::var("JWT_SECRET_KEY")
                        .expect("JWT_SECRET_KEY must be set")
                        .as_bytes(),
                ),
                &Validation::default(),
            );
            if let Ok(claims) = claims {
                let email = claims.claims.email;
                let user = user_repository::get_user(email);
                if let Some(user) = user {
                    return Outcome::Success(ValidatedUser {
                        email: user.email,
                        role: user.role,
                    });
                }
            }
        }
        return Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing));
    }
}
