use super::super::mysql::diesel_connection::establish_connection;
use crate::{
    domain::onsen::onsen_entity::OnsenEntity,
    infrastructure::mysql::diesel_model::{diesel_chemical::DieselChemical, diesel_onsen::Onsen},
    schema::{chemicals, onsen},
};
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
    let results: Vec<(Onsen, Option<DieselChemical>)> = query
        .left_join(chemicals::table)
        .select((Onsen::as_select(), Option::<DieselChemical>::as_select()))
        .load::<(Onsen, Option<DieselChemical>)>(connection)
        .expect("DB error");
    let onsen_entities = results
        .iter()
        .map(|v: &(Onsen, Option<DieselChemical>)| {
            let onsen_entity = OnsenEntity::create(v.0.clone(), v.1.clone());
            return onsen_entity;
        })
        .collect();
    onsen_entities
}

pub fn get_onsen(id: u32) -> Option<OnsenEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Onsen> = onsen::table
        .select(Onsen::as_select())
        .filter(onsen::dsl::id.eq(id))
        .load(connection)
        .expect("DB error");
    let onsen = results.first()?;
    Some(OnsenEntity::create(onsen.clone(), None))
}

pub fn put_onsen(onsen_entity: OnsenEntity) -> () {
    let updated_onsen = Onsen::from(onsen_entity);
    let connection = &mut establish_connection();
    let _ = diesel::update(onsen::table.find(updated_onsen.id))
        .set((
            onsen::dsl::name.eq(updated_onsen.name),
            onsen::dsl::spring_quality.eq(updated_onsen.spring_quality),
            onsen::dsl::liquid.eq(updated_onsen.liquid),
            onsen::dsl::osmotic_pressure.eq(updated_onsen.osmotic_pressure),
            onsen::dsl::category.eq(updated_onsen.category),
            onsen::dsl::day_use.eq(updated_onsen.day_use),
            onsen::dsl::url.eq(updated_onsen.url),
            onsen::dsl::description.eq(updated_onsen.description),
            onsen::dsl::hotel_id.eq(updated_onsen.hotel_id),
        ))
        .execute(connection)
        .expect("DB error");
}

pub fn put_onsen_description(id: u32, description: &str) -> () {
    let connection = &mut establish_connection();
    let _ = diesel::update(onsen::dsl::onsen.find(id))
        .set(onsen::dsl::description.eq(description))
        .execute(connection)
        .expect("DB error");
}

pub fn post_onsen(onsen_entity: OnsenEntity) -> OnsenEntity {
    let new_onsen = Onsen::from(onsen_entity);
    let connection = &mut establish_connection();
    diesel::insert_into(onsen::table)
        .values(&new_onsen)
        .execute(connection)
        .expect("DB error");
    OnsenEntity::create(new_onsen, None)
}
