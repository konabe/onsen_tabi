use crate::domain::area_entity::AreaEntity;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaResponse {
    pub id: u32,
    pub name: String,
    pub kana: String,
    pub prefecture: String,
    pub national_resort: bool,
    pub village: Option<String>,
    pub url: String,
    pub description: String,
    pub access: String,
    pub onsen_ids: Vec<u32>,
}

impl From<AreaEntity> for AreaResponse {
    fn from(value: AreaEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            kana: value.kana,
            prefecture: value.prefecture,
            national_resort: value.national_resort,
            village: value.village,
            url: value.url,
            description: value.description,
            access: value.access,
            onsen_ids: value.onsens.iter().map(|v| v.id).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::application::api_model::area_response::AreaResponse;
    use crate::domain::area_entity::AreaEntityBuilder;
    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::OnsenEntityBuilder;
    use crate::domain::onsen::onsen_quality::OnsenQuality;

    const COMMON_ONSEN_QUALITY: Lazy<OnsenQuality> =
        Lazy::new(|| OnsenQuality::new(&vec![NaIon, CaIon, SO4Ion], None));

    #[test]
    fn test_area_response() {
        let onsen = OnsenEntityBuilder::new()
            .id(2)
            .name("元禄の湯")
            .quality(Some(COMMON_ONSEN_QUALITY.clone()))
            .spring_quality("")
            .liquid(Some("neutral"))
            .osmotic_pressure(Some("hypotonic"))
            .temperature(Some("hot"))
            .form("uchiyu")
            .is_day_use(true)
            .url("https://www.sekizenkan.co.jp/spa/#ank-spa1")
            .img_url(Some("https://placehold.jp/150x150.png"))
            .description("")
            .area_id(None)
            .build()
            .expect("");
        let area = AreaEntityBuilder::new()
            .id(1)
            .name("四万")
            .kana("しま")
            .prefecture("群馬県")
            .national_resort(true)
            .village(None)
            .url("https://nakanojo-kanko.jp/shima/")
            .description("")
            .access("")
            .onsens(vec![onsen])
            .build()
            .expect("");
        let response = AreaResponse::from(area);
        assert_eq!(response.id, 1);
        assert_eq!(response.name, "四万");
        assert_eq!(response.kana, "しま");
        assert_eq!(response.prefecture, "群馬県");
        assert_eq!(response.national_resort, true);
        assert_eq!(response.village, None);
        assert_eq!(response.url, "https://nakanojo-kanko.jp/shima/");
        assert_eq!(response.description, "");
        assert_eq!(response.access, "");
        assert_eq!(response.onsen_ids, vec![2]);
    }
}
