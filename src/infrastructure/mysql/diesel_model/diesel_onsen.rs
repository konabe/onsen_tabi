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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::chemical::ClType;
    use crate::domain::onsen::onsen_quality::OnsenQuality;
    use crate::domain::onsen::spring_form::SpringForm;

    #[test]
    fn test_onsen_entity_create_from_diesel_onsen() {
        let diesel_onsen = Onsen {
            id: 1,
            name: "元禄の湯".to_string(),
            spring_quality: "ナトリウム－塩化物泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            category: "uchiyu".to_string(),
            day_use: true,
            url: "https://example.com".to_string(),
            img_url: Some("https://example.com/img.png".to_string()),
            description: "test description".to_string(),
            hotel_id: None,
            chemical_id: Some(1),
            area_id: Some(1),
        };

        let diesel_chemical = DieselChemical {
            id: 1,
            na_ion: 1,
            ca_ion: 0,
            mg_ion: 0,
            cl_ion: 4,
            hco3_ion: 0,
            so4_ion: 0,
            co2_ion: 0,
            fe_ion: 0,
            al_ion: 0,
            cu_ion: 0,
            h_ion: 0,
            i_ion: 0,
            s: 0,
            rn: 0,
            strong_na_cl: false,
            fe_type: "Normal".to_string(),
            weak_rn: false,
        };

        let entity = OnsenEntity::create(diesel_onsen, Some(diesel_chemical));

        assert_eq!(entity.id, 1);
        assert_eq!(entity.name, "元禄の湯");
        assert_eq!(entity.spring_quality, "ナトリウム－塩化物泉");
        assert_eq!(entity.form, SpringForm::Uchiyu);
        assert_eq!(entity.is_day_use, true);
        assert!(entity.quality.is_some());
    }

    #[test]
    fn test_onsen_entity_create_without_chemical() {
        let diesel_onsen = Onsen {
            id: 2,
            name: "単純温泉".to_string(),
            spring_quality: "単純温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: None,
            temperature: None,
            category: "sotoyu".to_string(),
            day_use: false,
            url: "https://example.com".to_string(),
            img_url: None,
            description: "".to_string(),
            hotel_id: Some(1),
            chemical_id: None,
            area_id: None,
        };

        let entity = OnsenEntity::create(diesel_onsen, None);

        assert_eq!(entity.id, 2);
        assert_eq!(entity.name, "単純温泉");
        assert_eq!(entity.form, SpringForm::Sotoyu);
        assert_eq!(entity.is_day_use, false);
        assert!(entity.quality.is_none());
    }

    #[test]
    fn test_diesel_onsen_from_onsen_entity() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Normal)], None);

        let entity = OnsenEntityBuilder::new()
            .id(1)
            .name("test onsen")
            .quality(Some(quality))
            .spring_quality("ナトリウム－塩化物泉")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("uchiyu")
            .is_day_use(true)
            .url("https://example.com")
            .img_url(Some("https://example.com/img.png"))
            .description("test")
            .area_id(Some(1))
            .build()
            .unwrap();

        let diesel: Onsen = entity.into();

        assert_eq!(diesel.id, 1);
        assert_eq!(diesel.name, "test onsen");
        assert_eq!(diesel.spring_quality, "ナトリウム－塩化物泉");
        assert_eq!(diesel.liquid, Some("neutral".to_string()));
        assert_eq!(diesel.osmotic_pressure, Some("hypotonic".to_string()));
        assert_eq!(diesel.temperature, Some("hot".to_string()));
        assert_eq!(diesel.category, "uchiyu");
        assert_eq!(diesel.day_use, true);
        assert_eq!(diesel.url, "https://example.com");
        assert_eq!(diesel.area_id, Some(1));
    }

    #[test]
    fn test_diesel_onsen_with_different_forms() {
        let forms = vec![
            ("uchiyu", SpringForm::Uchiyu),
            ("sotoyu", SpringForm::Sotoyu),
        ];

        for (form_str, expected_form) in forms {
            let diesel_onsen = Onsen {
                id: 1,
                name: "test".to_string(),
                spring_quality: "".to_string(),
                liquid: None,
                osmotic_pressure: None,
                temperature: None,
                category: form_str.to_string(),
                day_use: true,
                url: "https://example.com".to_string(),
                img_url: None,
                description: "".to_string(),
                hotel_id: None,
                chemical_id: None,
                area_id: None,
            };

            let entity = OnsenEntity::create(diesel_onsen, None);
            assert_eq!(entity.form, expected_form);
        }
    }

    #[test]
    fn test_diesel_onsen_with_different_liquids() {
        let liquids = vec![
            "acidic",
            "mildly_acidic",
            "neutral",
            "mildly_alkaline",
            "alkaline",
        ];

        for liquid_str in liquids {
            let diesel_onsen = Onsen {
                id: 1,
                name: "test".to_string(),
                spring_quality: "".to_string(),
                liquid: Some(liquid_str.to_string()),
                osmotic_pressure: None,
                temperature: None,
                category: "uchiyu".to_string(),
                day_use: true,
                url: "https://example.com".to_string(),
                img_url: None,
                description: "".to_string(),
                hotel_id: None,
                chemical_id: None,
                area_id: None,
            };

            let entity = OnsenEntity::create(diesel_onsen, None);
            assert!(entity.liquid.is_some());
        }
    }

    #[test]
    fn test_diesel_onsen_with_complex_chemical() {
        let diesel_chemical = DieselChemical {
            id: 1,
            na_ion: 1,
            ca_ion: 2,
            mg_ion: 0,
            cl_ion: 5,
            hco3_ion: 4,
            so4_ion: 0,
            co2_ion: 0,
            fe_ion: 7,
            al_ion: 0,
            cu_ion: 0,
            h_ion: 0,
            i_ion: 0,
            s: 0,
            rn: 0,
            strong_na_cl: false,
            fe_type: "Two".to_string(),
            weak_rn: false,
        };

        let diesel_onsen = Onsen {
            id: 1,
            name: "複雑な泉質".to_string(),
            spring_quality: "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            category: "uchiyu".to_string(),
            day_use: true,
            url: "https://example.com".to_string(),
            img_url: None,
            description: "".to_string(),
            hotel_id: None,
            chemical_id: Some(1),
            area_id: Some(1),
        };

        let entity = OnsenEntity::create(diesel_onsen, Some(diesel_chemical));

        assert!(entity.quality.is_some());
        let quality = entity.quality.unwrap();
        assert_eq!(quality.fe_type(), "Two");
        assert_eq!(
            quality.to_string(),
            "含鉄（Ⅱ）－ナトリウム・カルシウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_diesel_onsen_optional_fields() {
        let diesel_onsen = Onsen {
            id: 1,
            name: "minimal onsen".to_string(),
            spring_quality: "単純温泉".to_string(),
            liquid: None,
            osmotic_pressure: None,
            temperature: None,
            category: "uchiyu".to_string(),
            day_use: true,
            url: "https://example.com".to_string(),
            img_url: None,
            description: "".to_string(),
            hotel_id: None,
            chemical_id: None,
            area_id: None,
        };

        let entity = OnsenEntity::create(diesel_onsen, None);

        assert!(entity.liquid.is_none());
        assert!(entity.osmotic_pressure.is_none());
        assert!(entity.temperature.is_none());
        assert!(entity.img_url.is_none());
        assert!(entity.area_id.is_none());
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = OnsenEntityBuilder::new()
            .id(1)
            .name("test onsen")
            .quality(None)
            .spring_quality("単純温泉")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("uchiyu")
            .is_day_use(true)
            .url("https://example.com")
            .img_url(Some("https://example.com/img.png"))
            .description("test description")
            .area_id(Some(1))
            .build()
            .unwrap();

        let diesel: Onsen = original.clone().into();
        let restored = OnsenEntity::create(diesel, None);

        assert_eq!(original.id, restored.id);
        assert_eq!(original.name, restored.name);
        assert_eq!(original.spring_quality, restored.spring_quality);
        assert_eq!(original.form, restored.form);
        assert_eq!(original.is_day_use, restored.is_day_use);
        assert_eq!(original.url, restored.url);
        assert_eq!(original.description, restored.description);
    }
}
