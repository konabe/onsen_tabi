use diesel::*;

use crate::{domain::onsen_entity::OnsenEntity, schema::onsen};

use super::mysql::{diesel_connection::establish_connection, diesel_models::Onsen};

pub fn get_onsens() -> Vec<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Onsen> = onsen::table
        .select(Onsen::as_select())
        .load(connection)
        .expect("error");
    return results
        .iter()
        .map(|v: &Onsen| {
            OnsenEntity::new(
                v.id,
                &v.name,
                &v.spring_quality,
                v.liquid.as_deref(),
                v.osmotic_pressure.as_deref(),
                &v.category,
                &v.url,
                &v.description,
            )
            .expect("")
        })
        .collect();
}

pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Onsen> = onsen::table
        .select(Onsen::as_select())
        .load(connection)
        .expect("error");
    let result = results.iter().find(|r| r.id == id)?;
    OnsenEntity::new(
        result.id,
        &result.name,
        &result.spring_quality,
        result.liquid.as_deref(),
        result.osmotic_pressure.as_deref(),
        &result.category,
        &result.url,
        &result.description,
    )
}

pub fn put_onsen_description(id: u32, description: &str) -> () {
    let connection = &mut establish_connection();
    let _ = diesel::update(onsen::dsl::onsen.find(id))
        .set(onsen::dsl::description.eq(description))
        .execute(connection)
        .expect("");
}
