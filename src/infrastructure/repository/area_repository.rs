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
        .map(|v: &Area| AreaEntity::new(v.id, &v.name, &v.prefecture, &v.url).expect(""))
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
    let result = &results[0];
    AreaEntity::new(result.id, &result.name, &result.prefecture, &result.url)
}
