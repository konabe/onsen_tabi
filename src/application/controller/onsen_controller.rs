use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::application::api_model::onsen_api_model::*;
use crate::domain::onsen_entity::OnsenEntity;
use crate::infrastructure::onsen_repository;

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

#[post("/onsen", format = "json", data = "<onsen_req>")]
pub fn post_onsen(onsen_req: Json<OnsenRequest>) -> Result<Json<OnsenResponse>, Status> {
    let onsen_entity = OnsenEntity::new(
        0,
        &onsen_req.name,
        &onsen_req.sprint_quality,
        onsen_req.liquid.as_deref(),
        onsen_req.ostomic_pressure.as_deref(),
        &onsen_req.form,
        &onsen_req.url,
        &onsen_req.description,
    );
    if let Some(onsen_entity) = onsen_entity {
        let created_hotel = onsen_repository::post_onsen(onsen_entity);
        return Ok(Json(OnsenResponse::from(created_hotel.clone())));
    } else {
        return Err(Status::BadRequest);
    }
}
