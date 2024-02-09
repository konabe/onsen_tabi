use crate::domain::{
    area_entity::AreaEntity, hotel_entity::HotelEntity, onsen::onsen_entity::OnsenEntity,
};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

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

#[derive(Queryable, Selectable, Identifiable, Insertable, Associations, Debug, Clone)]
#[diesel(belongs_to(Hotel))]
#[diesel(table_name=crate::schema::onsen)]
pub struct Onsen {
    pub id: u32,
    pub name: String,
    pub spring_quality: String,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub category: String,
    pub day_use: Option<bool>,
    pub url: String,
    pub description: String,
    pub hotel_id: Option<u32>,
}

impl From<Onsen> for OnsenEntity {
    fn from(value: Onsen) -> Self {
        OnsenEntity::new(
            value.id,
            &value.name,
            &value.spring_quality,
            value.liquid.as_deref(),
            value.osmotic_pressure.as_deref(),
            &value.category,
            value.day_use,
            &value.url,
            &value.description,
        )
        .expect("Saved data violates OnsenEntity")
    }
}

impl From<OnsenEntity> for Onsen {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            spring_quality: value.spring_quality,
            liquid: value.liquid.map(|v| v.to_string()),
            osmotic_pressure: value.osmotic_pressure.map(|v| v.to_string()),
            category: value.form.to_string(),
            day_use: value.is_day_use,
            url: value.url,
            description: value.description,
            hotel_id: None,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::area)]
pub struct Area {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
}

impl From<Area> for AreaEntity {
    fn from(value: Area) -> Self {
        AreaEntity::new(
            value.id,
            &value.name,
            &value.prefecture,
            value.national_resort,
            value.village.as_deref(),
            &value.url,
            &value.description,
            vec![],
        )
        .expect("Saved data violates AreaEntity")
    }
}

impl From<AreaEntity> for Area {
    fn from(value: AreaEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            prefecture: value.prefecture,
            national_resort: value.national_resort,
            village: value.village,
            url: value.url,
            description: value.description,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::user)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}
