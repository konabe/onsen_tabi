use crate::domain::hotel_entity::HotelEntity;
use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub description: String,
}

impl From<Hotel> for HotelEntity {
    fn from(value: Hotel) -> Self {
        HotelEntity::new(
            value.id,
            &value.name,
            value.has_washitsu,
            value.url.as_str(),
            value.description.as_str(),
            &vec![],
        )
        .expect("Saved data violates HotelEntity")
    }
}

impl From<HotelEntity> for Hotel {
    fn from(value: HotelEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            has_washitsu: value.has_washitsu,
            description: value.description,
            url: value.url,
        }
    }
}
