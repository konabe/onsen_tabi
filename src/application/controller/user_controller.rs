use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::{
    application::api_model::user_api_model::{AuthRequest, AuthResponse},
    infrastructure::user_repository,
};

use argon2::{self, Config, Variant, Version};
use rand::{distributions::Alphanumeric, Rng};

#[post("/signup", format = "json", data = "<auth_req>")]
pub fn post_signup(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    // TODO: すでにemailが存在するか確認

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

    // TODO: emailをJWTに添加して署名
    Ok(Json(AuthResponse {
        token: "test".to_string(),
    }))
}
