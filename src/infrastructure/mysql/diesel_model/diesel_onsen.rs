use std::str::FromStr;

use crate::domain::onsen::onsen_entity::{OnsenEntity, OnsenEntityBuilder};
use crate::domain::onsen::spring_liquid::SpringLiquid;
use crate::infrastructure::mysql::diesel_model::{
    diesel_chemical::DieselChemical, diesel_hotel::Hotel,
};
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
        OnsenEntityBuilder::new()
            .id(onsen.id)
            .name(&onsen.name)
            .quality(onsen_quality)
            .spring_quality(&onsen.spring_quality)
            .liquid(onsen.liquid.as_deref())
            .osmotic_pressure(onsen.osmotic_pressure.as_deref())
            .temperature(onsen.temperature.as_deref())
            .form(&onsen.category)
            .is_day_use(onsen.day_use)
            .url(&onsen.url)
            .img_url(onsen.img_url.as_deref())
            .description(&onsen.description)
            .area_id(onsen.area_id)
            .build()
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
