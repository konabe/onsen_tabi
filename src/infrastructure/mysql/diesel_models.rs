use crate::domain::onsen::onsen_entity::OnsenEntity;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug)]
#[diesel(table_name=crate::schema::hotel)]
pub struct Hotel {
    pub id: u32,
    pub name: String,
    pub has_washitsu: bool,
    pub url: String,
    pub description: String,
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
            &value.url,
            &value.description,
        )
        .expect("Saved data violates OnsenEntity")
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug)]
#[diesel(table_name=crate::schema::area)]
pub struct Area {
    pub id: u32,
    pub name: String,
    pub prefecture: String,
    pub url: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, Clone)]
#[diesel(table_name=crate::schema::user)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}
