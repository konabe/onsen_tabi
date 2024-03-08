use crate::domain::onsen::{
    chemical::Chemical, onsen_entity::OnsenEntity, onsen_quality::OnsenQuality,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenRequest {
    pub name: String,
    pub spring_quality: String,
    pub chemicals: Option<OnsenChemicalsRequestModel>,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
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
    pub is_weak_rn: bool,
}

impl From<OnsenChemicalsRequestModel> for OnsenQuality {
    fn from(value: OnsenChemicalsRequestModel) -> Self {
        let mut chemicals: Vec<(Chemical, u32)> = vec![
            (Chemical::NaIon, value.na_ion),
            (Chemical::CaIon, value.ca_ion),
            (Chemical::MgIon, value.mg_ion),
            (Chemical::ClIon, value.cl_ion),
            (Chemical::HCO3Ion, value.hco3_ion),
            (Chemical::SO4Ion, value.so4_ion),
            (Chemical::CO2, value.co2_ion),
            (Chemical::FeIon(2), value.fe_ion),
            (Chemical::AlIon, value.al_ion),
            (Chemical::CuIon, value.cu_ion),
            (Chemical::HIon, value.h_ion),
            (Chemical::IIon, value.i_ion),
            (Chemical::S, value.s),
            (Chemical::Rn, value.rn),
        ]
        .into_iter()
        .filter(|(_, value)| *value > 0)
        .collect();
        chemicals.sort_by(|(_, a), (_, b)| a.cmp(b));
        let chemicals_values: Vec<Chemical> = chemicals
            .into_iter()
            .map(|(chemical, _)| chemical)
            .collect();
        Self::new(
            &chemicals_values,
            None,
            value.is_strong_na_cl,
            value.is_weak_rn,
        )
    }
}

impl OnsenRequest {
    pub fn create_entity(&self, id: u32) -> Option<OnsenEntity> {
        let quality = self.chemicals.clone().map(|v| OnsenQuality::from(v));
        OnsenEntity::new(
            id,
            self.name.as_str(),
            quality,
            self.spring_quality.as_str(),
            self.liquid.as_deref(),
            self.osmotic_pressure.as_deref(),
            self.temperature.as_deref(),
            self.form.as_str(),
            self.is_day_use,
            self.url.as_str(),
            self.img_url.as_deref(),
            self.description.as_str(),
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub spring_quality: String,
    pub quality: Option<OnsenQualityResponseModel>,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub temperature: Option<String>,
    pub form: String,
    pub is_day_use: bool,
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenQualityResponseModel {
    pub name: String,
    pub chemicals: Vec<String>,
    pub is_strong_na_cl: bool,
    pub is_weak_rn: bool,
}

impl From<OnsenEntity> for OnsenResponse {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            spring_quality: value.spring_quality.clone(),
            quality: value.quality.map(|v| OnsenQualityResponseModel {
                name: v.to_string(),
                chemicals: v.to_string_vec(),
                is_strong_na_cl: v.is_strong_na_cl,
                is_weak_rn: v.is_weak_rn,
            }),
            liquid: value.liquid.as_ref().map(|v| v.to_string()),
            osmotic_pressure: value.osmotic_pressure.as_ref().map(|v| v.to_string()),
            temperature: value.temperature.as_ref().map(|v| v.to_string()),
            form: value.form.to_string(),
            is_day_use: value.is_day_use,
            url: value.url.to_string(),
            img_url: value.img_url.as_ref().map(|v| v.to_string()),
            description: value.description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::api_model::onsen_api_model::{OnsenChemicalsRequestModel, OnsenRequest},
        domain::onsen::chemical::Chemical,
    };

    #[test]
    fn test_onsen_request_create_entity() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            spring_quality: "温泉法の温泉".to_string(),
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
                is_weak_rn: false,
            }),
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
        assert_eq!(quality.anions, vec![Chemical::HCO3Ion, Chemical::ClIon]);
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(2)]);
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
    fn test_onsen_request_create_entity_if_data_is_not_migrated() {
        let request = OnsenRequest {
            name: "元禄の湯".to_string(),
            spring_quality: "温泉法の温泉".to_string(),
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
                is_weak_rn: false,
            }),
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
        assert_eq!(quality.anions, vec![Chemical::ClIon, Chemical::HCO3Ion]);
        assert_eq!(quality.inclusions, vec![Chemical::FeIon(2)]);
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
