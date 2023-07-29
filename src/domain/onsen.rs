use super::{
    domain_error::DomainError,
    onsen_quality::{SpringForm, SpringLiquid, SpringOsmoticPressure},
};

/// 温泉法が定義する温泉。
/// ◯◯温泉とは別
pub struct Onsen {
    name: String,
    // TODO: 温泉法の定義に基づいてここをドメインモデル化できる
    spring_quality: String,
    liquid: SpringLiquid,
    osmotic_pressure: SpringOsmoticPressure,
    form: SpringForm,
}

impl Onsen {
    pub fn new(
        name: &str,
        spring_quality: &str,
        liquid: SpringLiquid,
        osmotic_pressure: SpringOsmoticPressure,
        form: SpringForm,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError);
        }
        return Ok(Self {
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
    let onsen = Onsen::new(
        "元禄の湯",
        "ナトリウム・カルシウム 塩化物硫酸塩温泉",
        SpringLiquid::Neutral,
        SpringOsmoticPressure::Hypotonic,
        SpringForm::Uchiyu,
    );
    let inside: Onsen = onsen.expect("");
    assert!(inside.name == "元禄の湯");
}
