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
    pub form: String,
    pub is_day_use: bool,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnsenChemicalsRequestModel {
    pub na_ion: bool,
    pub ca_ion: bool,
    pub mg_ion: bool,
    pub cl_ion: bool,
    pub hco3_ion: bool,
    pub so4_ion: bool,
    pub co2_ion: bool,
    pub fe_ion: bool,
    pub h_ion: bool,
    pub i_ion: bool,
    pub s: bool,
    pub rn: bool,
}

impl From<OnsenChemicalsRequestModel> for OnsenQuality {
    fn from(value: OnsenChemicalsRequestModel) -> Self {
        let mut chemicals: Vec<Chemical> = vec![];
        if value.na_ion {
            chemicals.push(Chemical::NaIon);
        }
        if value.ca_ion {
            chemicals.push(Chemical::CaIon)
        }
        if value.mg_ion {
            chemicals.push(Chemical::MgIon)
        }
        if value.cl_ion {
            chemicals.push(Chemical::ClIon)
        }
        if value.hco3_ion {
            chemicals.push(Chemical::HCO3Ion)
        }
        if value.so4_ion {
            chemicals.push(Chemical::SO4Ion)
        }
        if value.co2_ion {
            chemicals.push(Chemical::CO2)
        }
        if value.fe_ion {
            chemicals.push(Chemical::FeIon(2))
        }
        if value.h_ion {
            chemicals.push(Chemical::HIon)
        }
        if value.i_ion {
            chemicals.push(Chemical::IIon)
        }
        if value.s {
            chemicals.push(Chemical::S)
        }
        if value.rn {
            chemicals.push(Chemical::Rn)
        }
        Self::new(&chemicals, None)
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
            self.form.as_str(),
            self.is_day_use,
            self.url.as_str(),
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
    pub form: String,
    pub is_day_use: bool,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenQualityResponseModel {
    pub name: String,
    pub chemicals: Vec<String>,
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
            }),
            liquid: value.liquid.as_ref().map(|v| v.to_string()),
            osmotic_pressure: value.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: value.form.to_string(),
            is_day_use: value.is_day_use,
            url: value.url.to_string(),
            description: value.description.to_string(),
        }
    }
}
