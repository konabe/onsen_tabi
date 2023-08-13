use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::infrastructure::onsen_repository;

use super::api_model::OnsenResponse;

#[get("/onsen")]
pub fn get_onsens() -> Json<Vec<OnsenResponse>> {
    let onsens = onsen_repository::get_onsens();
    let response = onsens.iter().map(|r| OnsenResponse::create(r)).collect();
    Json(response)
}

#[get("/onsen/<onsen_id>")]
pub fn get_onsen(onsen_id: u32) -> Result<Json<OnsenResponse>, Status> {
    let other_onsen = onsen_repository::get_onsen(onsen_id);
    match other_onsen {
        Some(some_onsen) => Ok(Json(OnsenResponse::create(&some_onsen))),
        None => Err(Status::NotFound),
    }
}
