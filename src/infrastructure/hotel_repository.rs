use diesel::*;

use super::{mysql::diesel_connection::establish_connection, mysql::diesel_models::Hotel};
use crate::{
    domain::hotel_entity::HotelEntity,
    schema::hotel::{self},
};

pub fn get_hotels() -> Vec<HotelEntity> {
    let connection = &mut establish_connection();
    let results = hotel::table
        .select(Hotel::as_select())
        .load(connection)
        .expect("error");
    return results
        .iter()
        .map(|v: &Hotel| HotelEntity::new(v.id, &v.name, v.has_washitsu).expect(""))
        .collect();
}

pub fn get_hotel(id: u32) -> Option<HotelEntity> {
    let connection = &mut establish_connection();
    let results = hotel::table
        .select(Hotel::as_select())
        .load(connection)
        .expect("error");
    let result = results.iter().find(|r| r.id == id);
    match result {
        Some(result) => {
            Some(HotelEntity::new(result.id, &result.name, result.has_washitsu).expect(""))
        }
        None => None,
    }
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
    let hotel_entity =
        HotelEntity::new(new_hotel.id, &new_hotel.name, new_hotel.has_washitsu).expect("");
    return hotel_entity;
}
