use diesel::*;

use crate::{domain::area_entity::AreaEntity, schema::area};

use super::mysql::{diesel_connection::establish_connection, diesel_models::Area};

pub fn get_areas() -> Vec<AreaEntity> {
    let connection = &mut establish_connection();
    let results = area::table
        .select(Area::as_select())
        .load(connection)
        .expect("error");
    return results
        .iter()
        .map(|v: &Area| AreaEntity::new(v.id, &v.name).expect(""))
        .collect();
}
