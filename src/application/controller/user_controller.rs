use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::application::api_model::user_api_model::{AuthRequest, AuthResponse};

#[post("/signup", format = "json", data = "<auth_req>")]
pub fn post_signup(auth_req: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    Ok(Json(AuthResponse {
        token: "test".to_string(),
    }))
}
