use super::{diesel_chemical::DieselChemical, diesel_hotel::Hotel};
use crate::domain::onsen::{onsen_entity::OnsenEntity, onsen_quality::OnsenQuality};
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
    pub day_use: bool,
    pub url: String,
    pub description: String,
    pub hotel_id: Option<u32>,
    pub chemical_id: Option<u32>,
}

impl OnsenEntity {
    pub fn create(onsen: Onsen, diesel_chemical: Option<DieselChemical>) -> Self {
        let onsen_quality = diesel_chemical.map(|v| OnsenQuality::from(v));
        OnsenEntity::new(
            onsen.id,
            &onsen.name,
            onsen_quality,
            &onsen.spring_quality,
            onsen.liquid.as_deref(),
            onsen.osmotic_pressure.as_deref(),
            &onsen.category,
            onsen.day_use,
            &onsen.url,
            &onsen.description,
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
            chemical_id: None,
        }
    }
}
