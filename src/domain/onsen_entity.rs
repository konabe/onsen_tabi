use std::str::FromStr;

use super::onsen_quality::{SpringForm, SpringLiquid, SpringOsmoticPressure};

/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
pub struct OnsenEntity {
    pub id: u32,
    pub name: String,
    // TODO: 温泉法の定義に基づいてここをドメインモデル化できる
    pub spring_quality: String,
    pub liquid: Option<SpringLiquid>,
    pub osmotic_pressure: Option<SpringOsmoticPressure>,
    pub form: SpringForm,
}

impl OnsenEntity {
    pub fn new(
        id: u32,
        name: &str,
        spring_quality: &str,
        liquid: Option<&str>,
        osmotic_pressure: Option<&str>,
        form: &str,
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        let mut result_liquid = None;
        if let Some(some_liquid) = liquid {
            result_liquid = SpringLiquid::from_str(some_liquid).ok();
        }
        let mut result_osmotic_pressure = None;
        if let Some(some_osmotic_pressure) = osmotic_pressure {
            result_osmotic_pressure = SpringOsmoticPressure::from_str(some_osmotic_pressure).ok();
        }
        let form = SpringForm::from_str(form).ok()?;
        return Some(Self {
            id,
            name: name.to_string(),
            spring_quality: spring_quality.to_string(),
            liquid: result_liquid,
            osmotic_pressure: result_osmotic_pressure,
            form,
        });
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
    );
    onsen.expect("");
}
