use crate::{
    domain::{
        hotel_entity::{HotelEntity, HotelEntityBuilder},
        onsen::onsen_entity::OnsenEntity,
    },
    infrastructure::mysql::{
        diesel_connection::establish_connection,
        diesel_model::{diesel_hotel::Hotel, diesel_onsen::Onsen},
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
    let onsen_entities: Vec<OnsenEntity> = hotels_onsens
        .iter()
        .filter_map(|r| r.1.clone())
        .map(|onsen| OnsenEntity::create(onsen, None))
        .collect();

    HotelEntityBuilder::new()
        .id(hotel.id)
        .name(&hotel.name)
        .has_washitsu(hotel.has_washitsu)
        .solo_available(hotel.solo_available)
        .url(&hotel.url)
        .description(&hotel.description)
        .onsens(onsen_entities)
        .build()
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

pub fn put_hotel(hotel_entity: HotelEntity) {
    let updated_hotel = Hotel::from(hotel_entity);
    let connection = &mut establish_connection();
    let _ = diesel::update(hotel::dsl::hotel.find(updated_hotel.id))
        .set((
            hotel::dsl::name.eq(updated_hotel.name),
            hotel::dsl::has_washitsu.eq(updated_hotel.has_washitsu),
            hotel::dsl::solo_available.eq(updated_hotel.solo_available),
            hotel::dsl::description.eq(updated_hotel.description),
            hotel::dsl::url.eq(updated_hotel.url),
        ))
        .execute(connection)
        .expect("DB error");
}
