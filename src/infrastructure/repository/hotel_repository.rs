use crate::{
    domain::{hotel_entity::HotelEntity, onsen::onsen_entity::OnsenEntity},
    infrastructure::mysql::{
        diesel_connection::establish_connection,
        diesel_models::{Hotel, Onsen},
    },
    schema::{
        hotel::{self},
        onsen,
    },
};
use diesel::*;

pub fn get_hotels(area_id: Option<u32>) -> Vec<HotelEntity> {
    let connection = &mut establish_connection();
    let mut query = hotel::table.into_boxed();
    if let Some(area_id) = area_id {
        query = query.filter(hotel::dsl::area_id.eq(area_id));
    }
    let results: Vec<Hotel> = query
        .select(Hotel::as_select())
        .load(connection)
        .expect("DB error");
    return results
        .iter()
        .map(|v: &Hotel| HotelEntity::from(v.clone()))
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
    Some(HotelEntity::from(hotel.clone()))
}

pub fn post_hotel(hotel_entity: HotelEntity) -> HotelEntity {
    let new_hotel = Hotel::from(hotel_entity);
    let connection = &mut establish_connection();
    diesel::insert_into(hotel::table)
        .values(&new_hotel)
        .execute(connection)
        .expect("DB error");
    HotelEntity::from(new_hotel)
}

pub fn put_hotel(hotel_entity: HotelEntity) -> () {
    let updated_hotel = Hotel::from(hotel_entity);
    let connection = &mut establish_connection();
    let _ = diesel::update(hotel::dsl::hotel.find(updated_hotel.id))
        .set((
            hotel::dsl::name.eq(updated_hotel.name),
            hotel::dsl::has_washitsu.eq(updated_hotel.has_washitsu),
            hotel::dsl::description.eq(updated_hotel.description),
            hotel::dsl::url.eq(updated_hotel.url),
        ))
        .execute(connection)
        .expect("DB error");
}

pub fn put_hotel_description(id: u32, description: &str) -> () {
    let connection = &mut establish_connection();
    let _ = diesel::update(hotel::dsl::hotel.find(id))
        .set(hotel::dsl::description.eq(description))
        .execute(connection)
        .expect("DB error");
}
