use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::infrastructure::onsen_repository;

use super::api_model::{OnsenDescriptionRequest, OnsenResponse};

#[get("/onsen")]
pub fn get_onsens() -> Json<Vec<OnsenResponse>> {
    let onsens = onsen_repository::get_onsens();
    let response = onsens
        .iter()
        .map(|v| OnsenResponse::from(v.clone()))
        .collect();
    Json(response)
}

#[get("/onsen/<onsen_id>")]
pub fn get_onsen(onsen_id: u32) -> Result<Json<OnsenResponse>, Status> {
    let onsen = onsen_repository::get_onsen(onsen_id);
    match onsen {
        Some(onsen) => Ok(Json(OnsenResponse::from(onsen.clone()))),
        None => Err(Status::NotFound),
    }
}

#[put("/onsen/<onsen_id>/description", format = "json", data = "<req>")]
pub fn put_onsen_description(
    onsen_id: u32,
    req: Json<OnsenDescriptionRequest>,
) -> Result<(), Status> {
    onsen_repository::put_onsen_description(onsen_id, &req.description);
    Ok(())
}
