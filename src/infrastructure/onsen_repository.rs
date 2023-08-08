use diesel::*;

use crate::{domain::onsen_entity::OnsenEntity, schema::onsen};

use super::mysql::{diesel_connection::establish_connection, diesel_models::Onsen};

pub fn get_onsens() -> Vec<OnsenEntity> {
    let connection = &mut establish_connection();
    let results = onsen::table
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
                v.liquid.as_ref().map(|v| v.as_str()),
                v.osmotic_pressure.as_ref().map(|v| v.as_str()),
                &v.category,
            )
            .expect("")
        })
        .collect();
}

pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    let connection = &mut establish_connection();
    let results = onsen::table
        .select(Onsen::as_select())
        .load(connection)
        .expect("error");
    let result = results.iter().find(|r| r.id == id)?;
    OnsenEntity::new(
        result.id,
        &result.name,
        &result.spring_quality,
        result.liquid.as_ref().map(|v| v.as_str()),
        result.osmotic_pressure.as_ref().map(|v| v.as_str()),
        &result.category,
    )
}
