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
    let results: Vec<Hotel> = hotel::table
        .select(Hotel::as_select())
        .load(connection)
        .expect("DB error");
    return results
        .iter()
        .map(|v: &Hotel| {
            HotelEntity::new(v.id, &v.name, v.has_washitsu, &v.url, &vec![])
                .expect("Saved data violates HotelEntity")
        })
        .collect();
}

pub fn get_hotel_with_onsen(id: u32) -> Option<HotelEntity> {
    let connection = &mut establish_connection();
    let hotels_onsens: Vec<(Hotel, Option<Onsen>)> = hotel::table
        .left_join(onsen::table)
        .select((Hotel::as_select(), Option::<Onsen>::as_select()))
        .filter(hotel::dsl::id.eq(id))
        .load(connection)
        .expect("DB error");
    let hotel = &hotels_onsens.first()?.0;
    let related_onsens: Vec<&Option<Onsen>> = hotels_onsens.iter().map(|r| &r.1).collect();
    let mut onsen_entities: Vec<OnsenEntity> = vec![];
    for onsen in related_onsens {
        if let Some(onsen) = onsen {
            onsen_entities.push(OnsenEntity::from(onsen.clone()));
        }
    }
    Some(
        HotelEntity::new(
            hotel.id,
            &hotel.name,
            hotel.has_washitsu,
            &hotel.url,
            &onsen_entities,
        )
        .expect("Saved data violates HotelEntity"),
    )
}

pub fn post_hotel(hotel_enitty: HotelEntity) -> HotelEntity {
    let new_hotel = Hotel {
        id: 0,
        name: hotel_enitty.name,
        has_washitsu: hotel_enitty.has_washitsu,
        url: hotel_enitty.url,
    };
    let connection = &mut establish_connection();
    diesel::insert_into(hotel::table)
        .values(&new_hotel)
        .execute(connection)
        .expect("DB error");
    let hotel_entity = HotelEntity::new(
        new_hotel.id,
        &new_hotel.name,
        new_hotel.has_washitsu,
        &new_hotel.url,
        &vec![],
    )
    .expect("Saved data violates HotelEntity");
    return hotel_entity;
}
