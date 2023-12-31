use std::str::FromStr;
use strum_macros::{Display, EnumString};

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
    // TODO: 温泉法の定義に基づいてここをドメインモデル化できる
    pub spring_quality: String,
    pub liquid: Option<SpringLiquid>,
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    pub form: SpringForm,
    // TODO: データが移行完了したらOptionを外す。
    pub is_day_use: Option<bool>,
    pub url: String,
    pub description: String,
}

impl OnsenEntity {
    pub fn new(
        id: u32,
        name: &str,
        spring_quality: &str,
        liquid: Option<&str>,
        osmotic_pressure: Option<&str>,
        form: &str,
        is_day_use: Option<bool>,
        url: &str,
        description: &str,
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        let liquid = liquid.and_then(|v| SpringLiquid::from_str(v).ok());
        let osmotic_pressure =
            osmotic_pressure.and_then(|v| SpringOsmoticPressure::from_str(v).ok());
        let form = SpringForm::from_str(form).ok()?;
        Some(Self {
            id,
            name: name.to_string(),
            spring_quality: spring_quality.to_string(),
            liquid,
            osmotic_pressure,
            form,
            is_day_use,
            url: url.to_string(),
            description: description.to_string(),
        })
    }
}

#[test]
fn new_test() {
    let onsen = OnsenEntity::new(
        1,
        "元禄の湯",
        "ナトリウム・カルシウム 塩化物硫酸塩温泉",
        Some("neutral"),
        Some("hypotonic"),
        "uchiyu",
        Some(true),
        "https://www.sekizenkan.co.jp/spa/#ank-spa1",
        "",
    );
    let inside: OnsenEntity = onsen.expect("");
    assert!(inside.name == "元禄の湯");
}

#[test]
#[should_panic]
fn new_test_none() {
    let onsen = OnsenEntity::new(
        1,
        "",
        "ナトリウム・カルシウム 塩化物硫酸塩温泉",
        Some("neutral"),
        Some("hypotonic"),
        "uchiyu",
        Some(true),
        "https://www.sekizenkan.co.jp/spa/#ank-spa1",
        "",
    );
    onsen.expect("");
}
