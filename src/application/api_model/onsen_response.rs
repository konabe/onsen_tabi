use crate::domain::{area_entity::AreaEntity, onsen::onsen_entity::OnsenEntity};
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
    pub area: Option<OnsenAreaResponseModel>,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsenAreaResponseModel {
    pub id: u32,
    pub name: String,
}

impl OnsenResponse {
    pub fn create(onsen: OnsenEntity, area: Option<AreaEntity>) -> Self {
        Self {
            id: onsen.id,
            name: onsen.name.clone(),
            quality: onsen.quality.map(|v| OnsenQualityResponseModel {
                name: v.to_string(),
                chemicals: v.to_string_vec(),
                is_strong_na_cl: v.is_strong_na_cl(),
                fe_type: v.fe_type(),
                is_weak_rn: v.is_weak_rn(),
            }),
            other_spring_quality: onsen.spring_quality.clone(),
            liquid: onsen.liquid.as_ref().map(|v| v.to_string()),
            osmotic_pressure: onsen.osmotic_pressure.as_ref().map(|v| v.to_string()),
            temperature: onsen.temperature.as_ref().map(|v| v.to_string()),
            form: onsen.form.to_string(),
            is_day_use: onsen.is_day_use,
            url: onsen.url.to_string(),
            img_url: onsen.img_url.as_ref().map(|v| v.to_string()),
            description: onsen.description.to_string(),
            area: area.map(|v| OnsenAreaResponseModel {
                id: v.id,
                name: v.name.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::application::api_model::onsen_response::OnsenResponse;
    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::OnsenEntity;
    use crate::domain::onsen::onsen_quality::OnsenQuality;

    const COMMON_ONSEN_QUALITY: Lazy<OnsenQuality> =
        Lazy::new(|| OnsenQuality::new(&vec![NaIon, CaIon, SO4Ion], None));

    #[test]
    fn test_onsen_response_from_onsen_entity() {
        let onsen = OnsenEntity::new(
            1,
            "元禄の湯",
            Some(COMMON_ONSEN_QUALITY.clone()),
            "",
            Some("neutral"),
            Some("hypotonic"),
            Some("hot"),
            "uchiyu",
            true,
            "https://www.sekizenkan.co.jp/spa/#ank-spa1",
            Some("https://placehold.jp/150x150.png"),
            "",
            None,
        );
        let response: OnsenResponse = OnsenResponse::create(onsen.unwrap(), None);
        assert_eq!(response.name, "元禄の湯");
        assert_eq!(
            response.quality.unwrap().name,
            "ナトリウム・カルシウム－硫酸塩泉"
        );
        assert_eq!(response.liquid.unwrap(), "neutral");
        assert_eq!(response.osmotic_pressure.unwrap(), "hypotonic");
        assert_eq!(response.temperature.unwrap(), "hot");
        assert_eq!(response.form, "uchiyu");
        assert_eq!(response.is_day_use, true);
        assert_eq!(response.url, "https://www.sekizenkan.co.jp/spa/#ank-spa1");
        assert_eq!(
            response.img_url.unwrap(),
            "https://placehold.jp/150x150.png"
        );
        assert_eq!(response.description, "");
    }
}
