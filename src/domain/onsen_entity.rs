use super::onsen_quality::{SpringForm, SpringLiquid, SpringOsmoticPressure};

/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
pub struct OnsenEntity {
    pub name: String,
    // TODO: 温泉法の定義に基づいてここをドメインモデル化できる
    pub spring_quality: String,
    pub liquid: SpringLiquid,
    pub osmotic_pressure: SpringOsmoticPressure,
    pub form: SpringForm,
}

impl OnsenEntity {
    pub fn new(
        name: &str,
        spring_quality: &str,
        liquid: SpringLiquid,
        osmotic_pressure: SpringOsmoticPressure,
        form: SpringForm,
    ) -> Option<Self> {
        if name.is_empty() {
            return None;
        }
        return Some(Self {
            name: name.to_string(),
            spring_quality: spring_quality.to_string(),
            liquid,
            osmotic_pressure,
            form,
        });
    }
}

#[test]
fn new_test() {
    let onsen = OnsenEntity::new(
        "元禄の湯",
        "ナトリウム・カルシウム 塩化物硫酸塩温泉",
        SpringLiquid::Neutral,
        SpringOsmoticPressure::Hypotonic,
        SpringForm::Uchiyu,
    );
    let inside: OnsenEntity = onsen.expect("");
    assert!(inside.name == "元禄の湯");
}

#[test]
#[should_panic]
fn new_test_none() {
    let onsen = OnsenEntity::new(
        "",
        "ナトリウム・カルシウム 塩化物硫酸塩温泉",
        SpringLiquid::Neutral,
        SpringOsmoticPressure::Hypotonic,
        SpringForm::Uchiyu,
    );
    onsen.expect("");
}
