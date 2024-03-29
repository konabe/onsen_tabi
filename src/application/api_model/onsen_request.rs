use serde::Deserialize;
use std::str::FromStr;

use crate::domain::onsen::{
    chemical::{Chemical, ClType, FeType, RnType},
    onsen_entity::{OnsenEntity, SpringLiquid},
    onsen_quality::OnsenQuality,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenRequest {
    pub name: String,
    pub chemicals: Option<OnsenChemicalsRequestModel>,
    pub other_spring_quality: String,
    pub osmotic_pressure: Option<String>,
    pub liquid: Option<String>,
    pub temperature: Option<String>,
    pub form: String,
    pub is_day_use: bool,
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnsenChemicalsRequestModel {
    pub na_ion: u32,
    pub ca_ion: u32,
    pub mg_ion: u32,
    pub cl_ion: u32,
    pub hco3_ion: u32,
    pub so4_ion: u32,
    pub co2_ion: u32,
    pub fe_ion: u32,
    pub al_ion: u32,
    pub cu_ion: u32,
    pub h_ion: u32,
    pub i_ion: u32,
    pub s: u32,
    pub rn: u32,
    pub is_strong_na_cl: bool,
    pub fe_type: String,
    pub is_weak_rn: bool,
}

impl OnsenChemicalsRequestModel {
    fn create(&self, liquid: Option<String>) -> OnsenQuality {
        let cl_type: ClType = if self.is_strong_na_cl {
            ClType::Strong
        } else {
            ClType::Normal
        };
        let fe_type: FeType = if self.fe_type == "Two" {
            FeType::Two
        } else if self.fe_type == "Three" {
            FeType::Three
        } else {
            FeType::Normal
        };
        let rn_type: RnType = if self.is_weak_rn {
            RnType::Weak
        } else {
            RnType::Normal
        };
        let mut chemicals: Vec<(Chemical, u32)> = vec![
            (Chemical::NaIon, self.na_ion),
            (Chemical::CaIon, self.ca_ion),
            (Chemical::MgIon, self.mg_ion),
            (Chemical::ClIon(cl_type), self.cl_ion),
            (Chemical::HCO3Ion, self.hco3_ion),
            (Chemical::SO4Ion, self.so4_ion),
            (Chemical::CO2, self.co2_ion),
            (Chemical::FeIon(fe_type), self.fe_ion),
            (Chemical::AlIon, self.al_ion),
            (Chemical::CuIon, self.cu_ion),
            (Chemical::HIon, self.h_ion),
            (Chemical::IIon, self.i_ion),
            (Chemical::S, self.s),
            (Chemical::Rn(rn_type), self.rn),
        ]
        .into_iter()
        .filter(|(_, value)| *value > 0)
        .collect();
        chemicals.sort_by(|(_, a), (_, b)| a.cmp(b));
        let chemicals_values: Vec<Chemical> = chemicals
            .into_iter()
            .map(|(chemical, _)| chemical)
            .collect();
        let liquid = liquid.and_then(|v| SpringLiquid::from_str(v.as_str()).ok());
        OnsenQuality::new(&chemicals_values, liquid)
    }
}

impl OnsenRequest {
    pub fn create_entity(&self, id: u32) -> Option<OnsenEntity> {
        let quality = self
            .chemicals
            .clone()
            .map(|v| v.create(self.liquid.clone()));
        OnsenEntity::new(
            id,
            self.name.as_str(),
            quality,
            self.other_spring_quality.as_str(),
            self.liquid.as_deref(),
            self.osmotic_pressure.as_deref(),
            self.temperature.as_deref(),
            self.form.as_str(),
            self.is_day_use,
            self.url.as_str(),
            self.img_url.as_deref(),
            self.description.as_str(),
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::api_model::onsen_request::{OnsenChemicalsRequestModel, OnsenRequest},
        domain::onsen::chemical::{Chemical, ClType, FeType, RnType},
    };

    #[test]
    fn test_onsen_request_create_entity() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                is_strong_na_cl: false,
                fe_type: "Two".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        assert_eq!(entity.name, "元禄の湯");
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Two)]);
        assert_eq!(entity.spring_quality, "温泉法の温泉");
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
        assert_eq!(entity.is_day_use, true);
        assert_eq!(entity.url, "https://www.sekizenkan.co.jp/spa/#ank-spa1");
        assert_eq!(entity.img_url.unwrap(), "https://placehold.jp/150x150.png");
        assert_eq!(entity.description, "description");
    }

    #[test]
    fn test_onsen_request_create_entity_na_cl_strong() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                is_strong_na_cl: true,
                fe_type: "Two".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Strong)]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Two)]);
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物強塩泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_fe_type_normal() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                is_strong_na_cl: false,
                fe_type: "Normal".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Normal)]);
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_fe_type_two() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                is_strong_na_cl: false,
                fe_type: "Two".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Two)]);
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_fe_type_three() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                is_strong_na_cl: false,
                fe_type: "Three".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Three)]);
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅲ）－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_rn_is_weak() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                rn: 8,
                is_strong_na_cl: false,
                fe_type: "Two".to_string(),
                is_weak_rn: true,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(
            quality.inclusions,
            vec![Chemical::FeIon(FeType::Two), Chemical::Rn(RnType::Weak)]
        );
        assert_eq!(entity.spring_quality, "温泉法の温泉");
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）・弱放射能－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_rn_is_not_weak() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                na_ion: 2,
                ca_ion: 1,
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
                rn: 8,
                is_strong_na_cl: false,
                fe_type: "Two".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::CaIon, Chemical::NaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::HCO3Ion, Chemical::ClIon(ClType::Normal)]
        );
        assert_eq!(
            quality.inclusions,
            vec![Chemical::FeIon(FeType::Two), Chemical::Rn(RnType::Normal)]
        );
        assert_eq!(entity.spring_quality, "温泉法の温泉");
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）・放射能－カルシウム・ナトリウム－炭酸水素塩・塩化物泉"
        );
    }

    #[test]
    fn test_onsen_request_create_entity_if_data_is_not_migrated() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            chemicals: Some(OnsenChemicalsRequestModel {
                // 元々は含まれていれば1, そうでなければ0というデータが入っていた
                na_ion: 1,
                ca_ion: 1,
                mg_ion: 0,
                cl_ion: 1,
                hco3_ion: 1,
                so4_ion: 0,
                co2_ion: 0,
                fe_ion: 1,
                al_ion: 0,
                cu_ion: 0,
                h_ion: 0,
                i_ion: 0,
                s: 0,
                rn: 0,
                is_strong_na_cl: false,
                fe_type: "Two".to_string(),
                is_weak_rn: false,
            }),
            other_spring_quality: "温泉法の温泉".to_string(),
            liquid: Some("neutral".to_string()),
            osmotic_pressure: Some("hypotonic".to_string()),
            temperature: Some("hot".to_string()),
            form: "uchiyu".to_string(),
            is_day_use: true,
            url: "https://www.sekizenkan.co.jp/spa/#ank-spa1".to_string(),
            img_url: Some("https://placehold.jp/150x150.png".to_string()),
            description: "description".to_string(),
        };
        let entity = request.create_entity(1).unwrap();
        assert_eq!(entity.id, 1);
        assert_eq!(entity.name, "元禄の湯");
        let quality = entity.quality.clone().unwrap();
        assert_eq!(quality.cations, vec![Chemical::NaIon, Chemical::CaIon]);
        assert_eq!(
            quality.anions,
            vec![Chemical::ClIon(ClType::Normal), Chemical::HCO3Ion]
        );
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(FeType::Two)]);
        assert_eq!(entity.spring_quality, "温泉法の温泉");
        assert_eq!(
            entity.quality.unwrap().to_string(),
            "含鉄（Ⅱ）－ナトリウム・カルシウム－塩化物・炭酸水素塩泉"
        );
        assert_eq!(entity.is_day_use, true);
        assert_eq!(entity.url, "https://www.sekizenkan.co.jp/spa/#ank-spa1");
        assert_eq!(entity.img_url.unwrap(), "https://placehold.jp/150x150.png");
        assert_eq!(entity.description, "description");
    }
}
