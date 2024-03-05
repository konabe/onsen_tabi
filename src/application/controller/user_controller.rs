use crate::application::auth::crypto;
use crate::infrastructure::repository::user_repository;
use crate::{application::api_model::user_api_model::*, application::auth::jwt::encode_jwt};
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/signup", format = "json", data = "<auth_req>")]
pub fn post_signup(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let email = auth_req.email.as_str();
    let password = auth_req.password.as_str();

    let exists_user = user_repository::exists_user(email);
    if exists_user {
        return Err(Status::Conflict);
    }

    let hashed_password = crypto::create_hash(password);

    user_repository::post_user(email, hashed_password.as_str());

    Ok(Json(AuthResponse {
        token: encode_jwt(email),
    }))
}

#[post("/signin", format = "json", data = "<auth_req>")]
pub fn post_signin(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let email = auth_req.email.as_str();
    let password = auth_req.password.as_str();

    let user = user_repository::get_user(email);
    if user.is_none() {
        return Err(Status::Unauthorized);
    }
    let user = user.unwrap();

    let matches = crypto::verify_hash(password, user.hashed_password.as_str());
    if !matches {
        return Err(Status::Unauthorized);
    }

    Ok(Json(AuthResponse {
        token: encode_jwt(email),
    }))
}
