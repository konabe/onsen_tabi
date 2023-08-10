use diesel::*;

use super::{
    mysql::diesel_connection::establish_connection,
    mysql::diesel_models::{Hotel, Onsen},
};
use crate::{
    domain::{hotel_entity::HotelEntity, onsen_entity::OnsenEntity},
    schema::{
        hotel::{self},
        onsen,
    },
};

pub fn get_hotels() -> Vec<HotelEntity> {
    let connection = &mut establish_connection();
    let results = hotel::table
        .select(Hotel::as_select())
        .load(connection)
        .expect("error");
    return results
        .iter()
        .map(|v: &Hotel| HotelEntity::new(v.id, &v.name, v.has_washitsu, &vec![]).expect(""))
        .collect();
}

pub fn get_hotel(id: u32) -> Option<HotelEntity> {
    let connection = &mut establish_connection();
    let hotels_onsens: Vec<(Hotel, Option<Onsen>)> = hotel::table
        .left_join(onsen::table)
        .select((Hotel::as_select(), Option::<Onsen>::as_select()))
        .load::<(Hotel, Option<Onsen>)>(connection)
        .expect("");
    let result: Vec<&(Hotel, Option<Onsen>)> =
        hotels_onsens.iter().filter(|r| r.0.id == id).collect();
    if result.len() == 0 {
        return None;
    }
    let hotel = &result[0].0;
    let related_onsens: Vec<&Option<Onsen>> = result.iter().map(|r| &r.1).collect();
    let mut onsen_entities: Vec<OnsenEntity> = vec![];
    for o in related_onsens {
        if let Some(o) = o {
            if let Some(entity) = OnsenEntity::new(
                o.id,
                &o.name,
                &o.spring_quality,
                o.liquid.clone(),
                o.osmotic_pressure.clone(),
                &o.category,
            ) {
                onsen_entities.push(entity);
            }
        }
    }

    return Some(
        HotelEntity::new(hotel.id, &hotel.name, hotel.has_washitsu, &onsen_entities).expect(""),
    );
}

pub fn post_hotel(hotel_enitty: HotelEntity) -> HotelEntity {
    let new_hotel = Hotel {
        id: 0,
        name: hotel_enitty.name.clone(),
        has_washitsu: hotel_enitty.has_washitsu,
    };
    let connection = &mut establish_connection();
    diesel::insert_into(hotel::table)
        .values(&new_hotel)
        .execute(connection)
        .expect("error");
    let hotel_entity = HotelEntity::new(
        new_hotel.id,
        &new_hotel.name,
        new_hotel.has_washitsu,
        &vec![],
    )
    .expect("");
    return hotel_entity;
}
