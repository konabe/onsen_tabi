use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
    application::api_model::user_api_model::{AuthRequest, AuthResponse},
    infrastructure::user_repository,
};
use argon2::{self, Config, Variant, Version};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
    iat: i64,
    exp: i64,
}

#[post("/signup", format = "json", data = "<auth_req>")]
pub fn post_signup(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let exists_user = user_repository::exists_user(auth_req.email.clone());
    if exists_user {
        return Err(Status::Conflict);
    }

    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 1,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hashed_password =
        argon2::hash_encoded(&auth_req.password.as_bytes(), &salt.as_bytes(), &config).unwrap();

    user_repository::post_user(auth_req.email.clone(), hashed_password, salt);

    Ok(Json(AuthResponse {
        token: encode_jwt(&auth_req.email),
    }))
}

#[post("/signin", format = "json", data = "<auth_req>")]
pub fn post_signin(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let user = user_repository::get_user(auth_req.email.clone());
    if user.is_none() {
        return Err(Status::Unauthorized);
    }
    let user = user.unwrap();

    let matches =
        argon2::verify_encoded(&user.hashed_password, auth_req.password.as_bytes()).unwrap();
    if !matches {
        return Err(Status::Unauthorized);
    }

    Ok(Json(AuthResponse {
        token: encode_jwt(&auth_req.email),
    }))
}

fn encode_jwt(email: &str) -> String {
    let mut header = Header::default();
    header.typ = Some("JWT".to_string());
    header.alg = Algorithm::HS256;
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(24)).timestamp();
    let my_claims = Claims {
        email: email.to_string(),
        iat,
        exp,
    };
    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret("test".as_bytes()),
    )
    .unwrap()
}
