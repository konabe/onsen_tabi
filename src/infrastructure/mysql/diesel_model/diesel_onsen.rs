use super::diesel_hotel::Hotel;
use crate::domain::onsen::onsen_entity::OnsenEntity;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

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
