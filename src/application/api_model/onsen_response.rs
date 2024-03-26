use crate::domain::onsen::onsen_entity::OnsenEntity;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub quality: Option<OnsenQualityResponseModel>,
    pub other_spring_quality: String,
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
    pub fe_type: String,
    pub is_weak_rn: bool,
}

impl From<OnsenEntity> for OnsenResponse {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            quality: value.quality.map(|v| OnsenQualityResponseModel {
                name: v.to_string(),
                chemicals: v.to_string_vec(),
                is_strong_na_cl: v.is_strong_na_cl(),
                fe_type: v.fe_type(),
                is_weak_rn: v.is_weak_rn(),
            }),
            other_spring_quality: value.spring_quality.clone(),
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
