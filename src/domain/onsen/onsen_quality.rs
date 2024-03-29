use crate::domain::onsen::chemical::Chemical::{self, *};
use crate::domain::onsen::chemical::ClType;
use crate::domain::onsen::chemical::RnType;
use crate::domain::onsen::onsen_entity::SpringLiquid;
use crate::domain::onsen::onsen_entity::SpringLiquid::*;
use std::{fmt, vec};

use super::chemical::FeType;

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
        if chemicals.contains(&HIon) && liquid != Some(Acidic) {
            panic!("酸性泉は必ず液性は酸性である");
        }
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

    pub fn is_strong_na_cl(&self) -> bool {
        self.cations.contains(&NaIon) && self.anions.contains(&ClIon(ClType::Strong))
    }

    pub fn fe_type(&self) -> String {
        if self.inclusions.contains(&FeIon(FeType::Two)) {
            return "Two".to_string();
        }
        if self.inclusions.contains(&FeIon(FeType::Three)) {
            return "Three".to_string();
        }
        if self.inclusions.contains(&FeIon(FeType::Normal)) {
            return "Normal".to_string();
        }
        "".to_string()
    }

    pub fn is_weak_rn(&self) -> bool {
        self.inclusions.contains(&Rn(RnType::Weak))
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        let cations_string_vec: Vec<String> = self.cations.iter().map(|v| v.to_string()).collect();
        let anions_string_vec: Vec<String> = self.anions.iter().map(|v| v.to_string()).collect();
        let inclusions_string_vec: Vec<String> =
            self.inclusions.iter().map(|v| v.to_string()).collect();
        return [cations_string_vec, anions_string_vec, inclusions_string_vec].concat();
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
        let inclusion_h_ion_excluded = self
            .inclusions
            .iter()
            .filter(|&v| v.clone() != HIon)
            .collect::<Vec<&Chemical>>();
        let inclusion_enumerated_text = inclusion_h_ion_excluded
            .iter()
            .map(|v| v.jp())
            .collect::<Vec<String>>()
            .join("・");
        let mut text: String = format!("{}泉", anion_enumrated_text);
        if !self.cations.is_empty() {
            text = cation_enumrated_text + "－" + &text;
        }
        if !inclusion_h_ion_excluded.is_empty() {
            text = "含".to_owned() + &inclusion_enumerated_text + "－" + &text;
        }
        if self.inclusions.contains(&HIon) {
            text = "酸性－".to_owned() + &text;
        }
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::onsen::chemical::Chemical::*;
    use crate::domain::onsen::chemical::ClType;
    use crate::domain::onsen::chemical::FeType;
    use crate::domain::onsen::chemical::RnType;
    use crate::domain::onsen::onsen_quality::OnsenQuality;
    use crate::domain::onsen::onsen_quality::SpringLiquid::*;

    #[test]
    fn test_tanjun_onsen() {
        let quality = OnsenQuality::new(&vec![], Some(Neutral));
        assert_eq!(quality.to_string(), "単純温泉");
        let cloned_quality = quality.clone();
        assert_eq!(cloned_quality.to_string(), "単純温泉");
    }

    #[test]
    fn test_tanjun_onsen_if_no_liquid_is_given() {
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
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Normal)], None);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物泉");
    }

    #[test]
    fn test_strong_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Strong)], None);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物強塩泉");
    }

    #[test]
    fn test_strong_na_cl_onsen_if_other_anion_is_less() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Strong), SO4Ion], None);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物強塩・硫酸塩泉");
    }

    #[test]
    fn test_na_mg_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, MgIon, ClIon(ClType::Normal)], None);
        assert_eq!(quality.to_string(), "ナトリウム・マグネシウム－塩化物泉");
    }

    #[test]
    fn test_na_ca_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, CaIon, ClIon(ClType::Normal)], None);
        assert_eq!(quality.to_string(), "ナトリウム・カルシウム－塩化物泉");
    }

    #[test]
    fn test_ca_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![CaIon, HCO3Ion], None);
        assert_eq!(quality.to_string(), "カルシウム－炭酸水素塩泉");
    }

    #[test]
    fn test_na_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, HCO3Ion], None);
        assert_eq!(quality.to_string(), "ナトリウム－炭酸水素塩泉");
    }

    #[test]
    fn test_so4_onsen() {
        let quality = OnsenQuality::new(&vec![SO4Ion], None);
        assert_eq!(quality.to_string(), "硫酸塩泉");
    }

    #[test]
    fn test_mg_so4_onsen() {
        let quality = OnsenQuality::new(&vec![MgIon, SO4Ion], None);
        assert_eq!(quality.to_string(), "マグネシウム－硫酸塩泉");
    }

    #[test]
    fn test_na_so4_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, SO4Ion], None);
        assert_eq!(quality.to_string(), "ナトリウム－硫酸塩泉");
    }

    #[test]
    fn test_ca_so4_onsen() {
        let quality = OnsenQuality::new(&vec![CaIon, SO4Ion], None);
        assert_eq!(quality.to_string(), "カルシウム－硫酸塩泉");
    }

    #[test]
    fn test_co2_onsen() {
        let quality = OnsenQuality::new(&vec![CO2], None);
        assert_eq!(quality.to_string(), "単純二酸化炭素泉");
    }

    #[test]
    fn test_fe_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Two)], None);
        assert_eq!(quality.to_string(), "単純鉄泉");
    }

    #[test]
    fn test_fe_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Two), HCO3Ion], None);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－炭酸水素塩泉");
    }

    #[test]
    fn test_fe2_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Two), SO4Ion], None);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－硫酸塩泉");
    }

    #[test]
    fn test_fe3_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Three), SO4Ion], None);
        assert_eq!(quality.to_string(), "含鉄（Ⅲ）－硫酸塩泉");
    }

    #[test]
    fn test_fe_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Normal), SO4Ion], None);
        assert_eq!(quality.to_string(), "含鉄－硫酸塩泉");
    }

    #[test]
    fn test_al_onsen() {
        let quality = OnsenQuality::new(
            &vec![S, AlIon, FeIon(FeType::Two), NaIon, CaIon, SO4Ion],
            None,
        );
        assert_eq!(
            quality.to_string(),
            "含硫黄・アルミニウム・鉄（Ⅱ）－ナトリウム・カルシウム－硫酸塩泉"
        );
    }

    #[test]
    fn test_cu_onsen() {
        let quality =
            OnsenQuality::new(&vec![HIon, CuIon, FeIon(FeType::Two), SO4Ion], Some(Acidic));
        assert_eq!(quality.to_string(), "酸性－含銅・鉄（Ⅱ）－硫酸塩泉");
    }

    #[test]
    fn test_h_onsen() {
        let quality = OnsenQuality::new(&vec![HIon], Some(Acidic));
        assert_eq!(quality.to_string(), "単純酸性泉");
    }

    #[test]
    #[should_panic]
    fn test_h_onsen_without_acidic_liquid() {
        OnsenQuality::new(&vec![HIon], Some(Neutral));
    }

    #[test]
    fn test_h_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Normal), HIon], Some(Acidic));
        assert_eq!(quality.to_string(), "酸性－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_h_s_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Normal), HIon, S], Some(Acidic));
        // "酸性"はさらに仕切られる
        assert_eq!(quality.to_string(), "酸性－含硫黄－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_i_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![IIon, NaIon, ClIon(ClType::Normal)], None);
        assert_eq!(quality.to_string(), "含よう素－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_s_onsen() {
        let quality = OnsenQuality::new(&vec![S], None);
        assert_eq!(quality.to_string(), "単純硫黄泉");
    }

    #[test]
    fn test_rn_onsen() {
        let quality = OnsenQuality::new(&vec![Rn(RnType::Normal)], None);
        assert_eq!(quality.to_string(), "単純放射能泉");
    }

    #[test]
    fn test_weak_rn_onsen() {
        let quality = OnsenQuality::new(&vec![Rn(RnType::Weak)], None);
        assert_eq!(quality.to_string(), "単純弱放射能泉");
    }

    #[test]
    fn test_multi_inclusion_onsen() {
        let quality = OnsenQuality::new(&vec![S, FeIon(FeType::Two), NaIon, CaIon, SO4Ion], None);
        assert_eq!(
            quality.to_string(),
            // "含"は先頭にのみつける
            "含硫黄・鉄（Ⅱ）－ナトリウム・カルシウム－硫酸塩泉"
        );
    }

    #[test]
    fn test_is_strong_na_cl_false() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Normal)], None);
        assert_eq!(quality.is_strong_na_cl(), false);
    }

    #[test]
    fn test_is_strong_na_cl() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon(ClType::Strong)], None);
        assert_eq!(quality.is_strong_na_cl(), true);
    }

    #[test]
    fn test_is_fe_type_normal() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Normal)], None);
        assert_eq!(quality.fe_type(), "Normal".to_string());
    }

    #[test]
    fn test_is_fe_type_two() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Two)], None);
        assert_eq!(quality.fe_type(), "Two".to_string());
    }

    #[test]
    fn test_is_fe_type_three() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Three)], None);
        assert_eq!(quality.fe_type(), "Three".to_string());
    }

    #[test]
    fn test_is_fe_type_nothing() {
        let quality = OnsenQuality::new(&vec![], None);
        assert_eq!(quality.fe_type(), "".to_string());
    }

    #[test]
    fn test_is_weak_rn() {
        let quality = OnsenQuality::new(&vec![Rn(RnType::Weak)], None);
        assert_eq!(quality.is_weak_rn(), true);
    }

    #[test]
    fn test_is_weak_rn_false() {
        let quality = OnsenQuality::new(&vec![Rn(RnType::Normal)], None);
        assert_eq!(quality.is_weak_rn(), false);
    }

    #[test]
    fn test_is_weak_rn_not_contain() {
        let quality = OnsenQuality::new(&vec![], None);
        assert_eq!(quality.is_weak_rn(), false);
    }

    #[test]
    fn test_to_string_vec() {
        let quality = OnsenQuality::new(&vec![FeIon(FeType::Two), NaIon, HCO3Ion], None);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－ナトリウム－炭酸水素塩泉");
        assert_eq!(quality.to_string_vec(), vec!["NaIon", "HCO3Ion", "FeIon"]);
    }
}
