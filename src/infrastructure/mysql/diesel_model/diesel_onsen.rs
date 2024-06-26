use std::str::FromStr;

use super::{diesel_chemical::DieselChemical, diesel_hotel::Hotel};
use crate::domain::onsen::onsen_entity::{OnsenEntity, SpringLiquid};
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
    pub temperature: Option<String>,
    pub category: String,
    pub day_use: bool,
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
    pub hotel_id: Option<u32>,
    pub chemical_id: Option<u32>,
    pub area_id: Option<u32>,
}

impl OnsenEntity {
    pub fn create(onsen: Onsen, diesel_chemical: Option<DieselChemical>) -> Self {
        let liquid = onsen
            .liquid
            .clone()
            .and_then(|v| SpringLiquid::from_str(&v).ok());
        let onsen_quality = diesel_chemical.map(|v| v.create(liquid));
        OnsenEntity::new(
            onsen.id,
            &onsen.name,
            onsen_quality,
            &onsen.spring_quality,
            onsen.liquid.as_deref(),
            onsen.osmotic_pressure.as_deref(),
            onsen.temperature.as_deref(),
            &onsen.category,
            onsen.day_use,
            &onsen.url,
            onsen.img_url.as_deref(),
            &onsen.description,
            onsen.area_id,
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
            temperature: value.temperature.map(|v| v.to_string()),
            category: value.form.to_string(),
            day_use: value.is_day_use,
            url: value.url,
            img_url: value.img_url.map(|v| v.to_string()),
            description: value.description,
            hotel_id: None,
            chemical_id: None,
            area_id: value.area_id,
        }
    }
}
