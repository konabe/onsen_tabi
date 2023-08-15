use diesel::*;

use crate::{domain::onsen_entity::OnsenEntity, schema::onsen};

use super::mysql::{diesel_connection::establish_connection, diesel_models::Onsen};

pub fn get_onsens() -> Vec<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Onsen> = onsen::table
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
