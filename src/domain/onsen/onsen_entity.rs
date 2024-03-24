use std::str::FromStr;
use strum_macros::{Display, EnumString};

use super::onsen_quality::OnsenQuality;

/// 液性
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
pub enum SpringLiquid {
    #[strum(serialize = "acidic")]
    Acidic, // 酸性
    #[strum(serialize = "mildly_acidic")]
    MildlyAcidic, // 弱酸性
    #[strum(serialize = "neutral")]
    Neutral, // 中性
    #[strum(serialize = "mildly_alkaline")]
    MildlyAlkaline, // 弱アルカリ性
    #[strum(serialize = "alkaline")]
    Alkaline, // アルカリ性
}

/// 浸透圧
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
pub enum SpringOsmoticPressure {
    #[strum(serialize = "hypotonic")]
    Hypotonic, // 低張性
    #[strum(serialize = "isotonic")]
    Isotonic, // 等張性
    #[strum(serialize = "hypertonic")]
    Hypertonic, // 高張性
}

/// 温度
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
pub enum SpringTemperature {
    #[strum(serialize = "hot")]
    Hot, // 高温泉
    #[strum(serialize = "normal")]
    Warm, // 温泉
    #[strum(serialize = "cool")]
    Cool, // 低温泉
    #[strum(serialize = "cold")]
    Cold, // 冷鉱泉
}

/// 営業形態
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
pub enum SpringForm {
    #[strum(serialize = "uchiyu")]
    Uchiyu, // 内湯
    #[strum(serialize = "sotoyu")]
    Sotoyu, // 外湯
}

/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
#[derive(Clone)]
pub struct OnsenEntity {
    pub id: u32,
    pub name: String,
    // TODO: "その他の泉質"を格納する場所を作る
    pub quality: Option<OnsenQuality>,
    pub spring_quality: String,
    pub liquid: Option<SpringLiquid>,
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    pub temperature: Option<SpringTemperature>,
    pub form: SpringForm,
    pub is_day_use: bool,
    pub url: String,
    pub img_url: Option<String>,
    pub description: String,
}

impl OnsenEntity {
    pub fn new(
        id: u32,
        name: &str,
        quality: Option<OnsenQuality>,
        spring_quality: &str,
        liquid: Option<&str>,
        osmotic_pressure: Option<&str>,
        temperature: Option<&str>,
        form: &str,
        is_day_use: bool,
        url: &str,
        img_url: Option<&str>,
        description: &str,
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        let liquid = liquid.and_then(|v| SpringLiquid::from_str(v).ok());
        let osmotic_pressure =
            osmotic_pressure.and_then(|v| SpringOsmoticPressure::from_str(v).ok());
        let temperature = temperature.and_then(|v| SpringTemperature::from_str(v).ok());
        let form = SpringForm::from_str(form).ok()?;
        Some(Self {
            id,
            name: name.to_string(),
            quality,
            spring_quality: spring_quality.to_string(),
            liquid,
            osmotic_pressure,
            temperature,
            form,
            is_day_use,
            url: url.to_string(),
            img_url: img_url.map(|v| v.to_string()),
            description: description.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::onsen_entity::OnsenEntity;
    use crate::domain::onsen::onsen_quality::OnsenQuality;

    const COMMON_ONSEN_QUALITY: Lazy<OnsenQuality> =
        Lazy::new(|| OnsenQuality::new(&vec![NaIon, CaIon, SO4Ion], None, false));

    #[test]
    fn new_test() {
        let onsen = OnsenEntity::new(
            1,
            "元禄の湯",
            Some(COMMON_ONSEN_QUALITY.clone()),
            "ナトリウム・カルシウム 塩化物硫酸塩温泉",
            Some("neutral"),
            Some("hypotonic"),
            Some("hot"),
            "uchiyu",
            true,
            "https://www.sekizenkan.co.jp/spa/#ank-spa1",
            Some("https://placehold.jp/150x150.png"),
            "",
        );
        let inside: OnsenEntity = onsen.expect("");
        assert!(inside.name == "元禄の湯");
    }

    #[test]
    #[should_panic]
    fn new_test_return_none_when_name_is_empty() {
        let onsen = OnsenEntity::new(
            1,
            "",
            Some(COMMON_ONSEN_QUALITY.clone()),
            "ナトリウム・カルシウム 塩化物硫酸塩温泉",
            Some("neutral"),
            Some("hypotonic"),
            Some("hot"),
            "uchiyu",
            true,
            "https://www.sekizenkan.co.jp/spa/#ank-spa1",
            Some("https://placehold.jp/150x150.png"),
            "",
        );
        onsen.expect("");
    }
}
