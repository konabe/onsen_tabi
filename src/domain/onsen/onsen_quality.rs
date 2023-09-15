use super::chemical::Chemical::{self, *};
use super::onsen_entity::SpringLiquid;
use super::onsen_entity::SpringLiquid::*;
use std::{fmt, vec};

#[derive(Clone)]
pub struct OnsenQuality {
    is_simple: bool,
    liquid: Option<SpringLiquid>,
    pub cations: Vec<Chemical>,
    pub anions: Vec<Chemical>,
    pub inclusions: Vec<Chemical>,
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
                inclusions: vec![],
            };
        }
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
        let inclusions: Vec<Chemical> = chemicals
            .iter()
            .filter(|v| v.is_inclusion())
            .map(|v| v.clone())
            .collect();

        let is_simple = cations.is_empty() && anions.is_empty();
        Self {
            is_simple,
            liquid,
            cations,
            anions,
            inclusions,
        }
    }

    fn liquid_string(&self) -> String {
        let empty = "".to_string();
        match &self.liquid {
            Some(liquid) => match liquid {
                Acidic | MildlyAcidic | Neutral => empty,
                MildlyAlkaline => "弱アルカリ性".to_string(),
                Alkaline => "アルカリ性".to_string(),
            },
            None => "".to_string(),
        }
    }
}

impl fmt::Display for OnsenQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_simple && self.inclusions.is_empty() {
            return write!(f, "{}単純温泉", self.liquid_string());
        }
        if self.is_simple && !self.inclusions.is_empty() {
            let target = &self.inclusions[0];
            let name = if let FeIon(_) = target {
                "鉄".to_string()
            } else {
                target.jp()
            };
            return write!(f, "単純{}泉", name);
        }
        let cation_enumrated_text = self
            .cations
            .iter()
            .map(|v| v.jp())
            .collect::<Vec<String>>()
            .join("・");
        let anion_enumrated_text = self
            .anions
            .iter()
            .map(|v| v.jp())
            .collect::<Vec<String>>()
            .join("・");
        let inclusion_enumerated_text = self
            .inclusions
            .iter()
            .map(|v| format!("含{}", v.jp()))
            .collect::<Vec<String>>()
            .join("・");
        let mut text: String = format!("{}泉", anion_enumrated_text);
        if !self.cations.is_empty() {
            text = cation_enumrated_text + "－" + &text;
        }
        if !self.inclusions.is_empty() {
            text = inclusion_enumerated_text + "－" + &text;
        }
        write!(f, "{}", text)
    }
}

#[test]
fn test_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], None);
    assert_eq!(quality.to_string(), "単純温泉");
}

#[test]
fn test_mildly_alkaline_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], Some(MildlyAlkaline));
    assert_eq!(quality.to_string(), "弱アルカリ性単純温泉");
}

#[test]
fn test_alkaline_tanjun_onsen() {
    let quality = OnsenQuality::new(&vec![], Some(Alkaline));
    assert_eq!(quality.to_string(), "アルカリ性単純温泉");
}

#[test]
fn test_na_cl_onsen() {
    let quality = OnsenQuality::new(&vec![NaIon, ClIon], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "ナトリウム－塩化物泉");
}

#[test]
fn test_na_mg_cl_onsen() {
    let quality = OnsenQuality::new(&vec![NaIon, MgIon, ClIon], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "ナトリウム・マグネシウム－塩化物泉");
}

#[test]
fn test_na_ca_cl_onsen() {
    let quality = OnsenQuality::new(&vec![NaIon, CaIon, ClIon], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "ナトリウム・カルシウム－塩化物泉");
}

#[test]
fn test_ca_hco3_onsen() {
    let quality = OnsenQuality::new(&vec![CaIon, HCO3Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "カルシウム－炭酸水素塩泉");
}

#[test]
fn test_na_hco3_onsen() {
    let quality = OnsenQuality::new(&vec![NaIon, HCO3Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "ナトリウム－炭酸水素塩泉");
}

#[test]
fn test_so4_onsen() {
    let quality = OnsenQuality::new(&vec![SO4Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "硫酸塩泉");
}

#[test]
fn test_mg_so4_onsen() {
    let quality = OnsenQuality::new(&vec![MgIon, SO4Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "マグネシウム－硫酸塩泉");
}

#[test]
fn test_na_so4_onsen() {
    let quality = OnsenQuality::new(&vec![NaIon, SO4Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "ナトリウム－硫酸塩泉");
}

#[test]
fn test_ca_so4_onsen() {
    let quality = OnsenQuality::new(&vec![CaIon, SO4Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "カルシウム－硫酸塩泉");
}

#[test]
fn test_co2_onsen() {
    let quality = OnsenQuality::new(&vec![CO2], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "単純二酸化炭素泉");
}

#[test]
fn test_fe_onsen() {
    let quality = OnsenQuality::new(&vec![FeIon(2)], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "単純鉄泉");
}

#[test]
fn test_fe_hco3_onsen() {
    let quality = OnsenQuality::new(&vec![FeIon(2), HCO3Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "含鉄（Ⅱ）－炭酸水素塩泉");
}

#[test]
fn test_fe_so4_onsen() {
    let quality = OnsenQuality::new(&vec![FeIon(2), SO4Ion], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "含鉄（Ⅱ）－硫酸塩泉");
}

#[test]
fn test_h_onsen() {
    let quality = OnsenQuality::new(&vec![HIon], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "単純酸性泉");
}

#[test]
fn test_i_na_cl_onsen() {
    let quality = OnsenQuality::new(&vec![IIon, NaIon, ClIon], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "含よう素－ナトリウム－塩化物泉");
}

#[test]
fn test_s_onsen() {
    let quality = OnsenQuality::new(&vec![S], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "単純硫黄泉");
}

#[test]
fn test_rn_onsen() {
    let quality = OnsenQuality::new(&vec![Rn], None);
    println!("{}", quality.to_string());
    assert_eq!(quality.to_string(), "単純放射能泉");
}
