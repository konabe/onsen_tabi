use super::mysql::{diesel_connection::establish_connection, diesel_models::Onsen};
use crate::{domain::onsen_entity::OnsenEntity, schema::onsen};
use diesel::*;

pub fn get_onsens(area_id: Option<u32>, hotel_id: Option<u32>) -> Vec<OnsenEntity> {
    let connection = &mut establish_connection();
    let mut query = onsen::table.into_boxed();
    if let Some(area_id) = area_id {
        query = query.filter(onsen::dsl::area_id.eq(area_id));
    }
    if let Some(hotel_id) = hotel_id {
        query = query.filter(onsen::dsl::hotel_id.eq(hotel_id));
    }
    let results: Vec<Onsen> = query
        .select(Onsen::as_select())
        .load(connection)
        .expect("DB error");
    return results
        .iter()
        .map(|v: &Onsen| OnsenEntity::from(v.clone()))
        .collect();
}

pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Onsen> = onsen::table
        .select(Onsen::as_select())
        .filter(onsen::dsl::id.eq(id))
        .load(connection)
        .expect("DB error");
    let onsen = results.first()?;
    Some(OnsenEntity::from(onsen.clone()))
}

pub fn put_onsen_description(id: u32, description: &str) -> () {
    let connection = &mut establish_connection();
    let _ = diesel::update(onsen::dsl::onsen.find(id))
        .set(onsen::dsl::description.eq(description))
        .execute(connection)
        .expect("DB error");
}

pub fn post_onsen(onsen_entity: OnsenEntity) -> OnsenEntity {
    let new_onsen = Onsen {
        id: 0,
        name: onsen_entity.name,
        spring_quality: onsen_entity.spring_quality,
        liquid: onsen_entity.liquid.map(|v| v.to_string()),
        osmotic_pressure: onsen_entity.osmotic_pressure.map(|v| v.to_string()),
        category: onsen_entity.form.to_string(),
        url: onsen_entity.url,
        description: onsen_entity.description,
        hotel_id: None,
    };
    let connection = &mut establish_connection();
    diesel::insert_into(onsen::table)
        .values(&new_onsen)
        .execute(connection)
        .expect("DB error");
    OnsenEntity::from(new_onsen)
}
