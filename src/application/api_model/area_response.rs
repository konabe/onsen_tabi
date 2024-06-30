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
    use crate::domain::area_entity::AreaEntity;
    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::OnsenEntity;
    use crate::domain::onsen::onsen_quality::OnsenQuality;

    const COMMON_ONSEN_QUALITY: Lazy<OnsenQuality> =
        Lazy::new(|| OnsenQuality::new(&vec![NaIon, CaIon, SO4Ion], None));

    #[test]
    fn test_area_response() {
        let onsen = OnsenEntity::new(
            2,
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
        )
        .expect("");
        let area = AreaEntity::new(
            1,
            "四万",
            "しま",
            "群馬県",
            true,
            None,
            "https://nakanojo-kanko.jp/shima/",
            "",
            "",
            vec![onsen],
        )
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
