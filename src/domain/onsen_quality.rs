use std::{fmt, vec};

use super::{chemical::Chemical, onsen_entity::SpringLiquid};

struct OnsenQuality {
    is_simple: bool,
    liquid: Option<SpringLiquid>,
    cations: Vec<Chemical>,
    anions: Vec<Chemical>,
    inclusion: Vec<Chemical>,
}

// https://www.env.go.jp/nature/onsen/pdf/2-5_p_16.pdf
impl OnsenQuality {
    pub fn new(chemicals: &[Chemical], liquid: Option<SpringLiquid>) -> Self {
        if chemicals.is_empty() {
            return Self {
                is_simple: true,
                liquid,
                cations: vec![],
                anions: vec![],
                inclusion: vec![],
            };
        }
        let is_simple = false;
        let cations: Vec<Chemical> = chemicals
            .iter()
            .filter(|v| v.is_cation())
            .map(|v| v.clone())
            .collect();
        let anions: Vec<Chemical> = chemicals
            .iter()
            .filter(|v| v.is_anion())
            .map(|v| v.clone())
            .collect();
        Self {
            is_simple,
            liquid,
            cations,
            anions,
            inclusion: vec![],
        }
    }

    fn liquid_string(&self) -> String {
        let empty = "".to_string();
        match &self.liquid {
            Some(liquid) => match liquid {
                SpringLiquid::Acidic => empty,
                SpringLiquid::MildlyAcidic => empty,
                SpringLiquid::Neutral => empty,
                SpringLiquid::MildlyAlkaline => "弱アルカリ性".to_string(),
                SpringLiquid::Alkaline => "アルカリ性".to_string(),
            },
            None => "".to_string(),
        }
    }
}

impl fmt::Display for OnsenQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_simple {
            return write!(f, "{}単純温泉", self.liquid_string());
        }
        write!(
            f,
            "{}－{}泉",
            self.cations
                .iter()
                .map(|v| v.jp())
                .collect::<Vec<String>>()
                .join("・"),
            self.anions
                .iter()
                .map(|v| v.jp())
                .collect::<Vec<String>>()
                .join("・"),
        )
    }
}

#[test]
fn test_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], None);
    assert!(quality.to_string() == "単純温泉");
}

#[test]
fn test_mildly_alkaline_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], Some(SpringLiquid::MildlyAlkaline));
    assert!(quality.to_string() == "弱アルカリ性単純温泉");
}

#[test]
fn test_alkaline_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], Some(SpringLiquid::Alkaline));
    assert!(quality.to_string() == "アルカリ性単純温泉");
}

#[test]
fn test_na_cl_onsen() {
    let quality = OnsenQuality::new(&vec![Chemical::NaIon, Chemical::ClIon], None);
    println!("{}", quality.to_string());
    assert!(quality.to_string() == "ナトリウム－塩化物泉");
}
