use crate::domain::onsen::onsen_entity::OnsenEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenRequest {
    pub name: String,
    pub spring_quality: String,
    pub chemicals: Option<Vec<String>>,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub form: String,
    pub is_day_use: Option<bool>,
    pub url: String,
    pub description: String,
}

impl OnsenRequest {
    pub fn create_entity(&self, id: u32) -> Option<OnsenEntity> {
        OnsenEntity::new(
            id,
            self.name.as_str(),
            None,
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
pub struct OnsenQualityResponse {
    pub name: String,
    pub chemicals: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub spring_quality: String,
    pub quality: Option<OnsenQualityResponse>,
    pub liquid: Option<String>,
    pub osmotic_pressure: Option<String>,
    pub form: String,
    pub is_day_use: Option<bool>,
    pub url: String,
    pub description: String,
}

impl From<OnsenEntity> for OnsenResponse {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            spring_quality: value.spring_quality.clone(),
            quality: value.quality.map(|v| OnsenQualityResponse {
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
