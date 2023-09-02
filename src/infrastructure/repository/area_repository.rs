use super::super::mysql::{diesel_connection::establish_connection, diesel_models::Area};
use crate::{domain::area_entity::AreaEntity, schema::area};
use diesel::*;

pub fn get_areas() -> Vec<AreaEntity> {
    let connection = &mut establish_connection();
    let results = area::table
        .select(Area::as_select())
        .load(connection)
        .expect("error");
    return results
        .iter()
        .map(|v: &Area| AreaEntity::from(v.clone()))
        .collect();
}

pub fn get_area(id: u32) -> Option<AreaEntity> {
    let connection = &mut establish_connection();
    let results: Vec<Area> = area::table
        .select(Area::as_select())
        .filter(area::dsl::id.eq(id))
        .load(connection)
        .expect("error");
    if results.len() == 0 {
        return None;
    }
    let area = results.first()?;
    Some(AreaEntity::from(area.clone()))
}

pub fn put_area(area_entity: AreaEntity) -> () {
    let updated_area = Area::from(area_entity);
    let connection = &mut establish_connection();
    let _ = diesel::update(area::table.find(updated_area.id))
        .set((
            area::dsl::name.eq(updated_area.name),
            area::dsl::prefecture.eq(updated_area.prefecture),
            area::dsl::national_resort.eq(updated_area.national_resort),
            area::dsl::village.eq(updated_area.village),
            area::dsl::url.eq(updated_area.url),
            area::dsl::description.eq(updated_area.description),
        ))
        .execute(connection)
        .expect("DB error");
}

pub fn put_area_description(id: u32, description: &str) -> () {
    let connection = &mut establish_connection();
    let _ = diesel::update(area::dsl::area.find(id))
        .set(area::dsl::description.eq(description))
        .execute(connection)
        .expect("DB error");
}

pub fn post_area(area_entity: AreaEntity) -> AreaEntity {
    let new_area = Area::from(area_entity);
    let connection = &mut establish_connection();
    diesel::insert_into(area::table)
        .values(&new_area)
        .execute(connection)
        .expect("DB error");
    AreaEntity::from(new_area)
}
