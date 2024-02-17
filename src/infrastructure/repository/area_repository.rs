use crate::infrastructure::mysql::{
    diesel_connection::establish_connection, diesel_model::diesel_area::Area,
    diesel_model::diesel_onsen::Onsen,
};
use crate::{
    domain::area_entity::AreaEntity, domain::onsen::onsen_entity::OnsenEntity, schema::area,
    schema::onsen,
};
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

pub fn get_areas_with_onsen() -> Vec<AreaEntity> {
    let connection = &mut establish_connection();
    let areas_onsens: Vec<(Area, Option<Onsen>)> = area::table
        .left_join(onsen::table)
        .select((Area::as_select(), Option::<Onsen>::as_select()))
        .load(connection)
        .expect("DB error");
    let mut area_entities: Vec<AreaEntity> = vec![];
    for area_onsen in areas_onsens {
        let (area, onsen) = area_onsen;
        let got_area = area_entities.iter_mut().find(|v| v.id == area.id);
        if let Some(area_entity) = got_area {
            if let Some(onsen) = onsen {
                area_entity.onsens.push(OnsenEntity::create(onsen, None));
            }
        } else {
            let mut area_entity = AreaEntity::from(area);
            if let Some(onsen) = onsen {
                area_entity.onsens.push(OnsenEntity::create(onsen, None));
            }
            area_entities.push(area_entity);
        }
    }
    area_entities
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

pub fn post_area(area_entity: AreaEntity) -> AreaEntity {
    let new_area = Area::from(area_entity);
    let connection = &mut establish_connection();
    diesel::insert_into(area::table)
        .values(&new_area)
        .execute(connection)
        .expect("DB error");
    AreaEntity::from(new_area)
}
