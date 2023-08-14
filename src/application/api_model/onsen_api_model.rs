use serde::{Deserialize, Serialize};

use crate::domain::onsen_entity::OnsenEntity;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenDescriptionRequest {
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenResponse {
    pub id: u32,
    pub name: String,
    pub sprint_quality: String,
    pub liquid: Option<String>,
    pub ostomic_pressure: Option<String>,
    pub form: String,
    pub description: String,
}

impl From<OnsenEntity> for OnsenResponse {
    fn from(value: OnsenEntity) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            sprint_quality: value.spring_quality.clone(),
            liquid: value.liquid.as_ref().map(|v| v.to_string()),
            ostomic_pressure: value.osmotic_pressure.as_ref().map(|v| v.to_string()),
            form: value.form.to_string(),
            description: value.description.to_string(),
        }
    }
}
