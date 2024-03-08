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
    pub is_strong_na_cl: bool,
    pub is_weak_rn: bool,
}

// https://www.env.go.jp/nature/onsen/pdf/2-5_p_16.pdf
impl OnsenQuality {
    pub fn new(
        chemicals: &[Chemical],
        liquid: Option<SpringLiquid>,
        is_strong_na_cl: bool,
        is_weak_rn: bool,
    ) -> Self {
        if liquid == Some(Acidic) && !chemicals.contains(&HIon)
            || chemicals.contains(&HIon) && liquid != Some(Acidic)
        {
            panic!("酸性の温泉には必ず水素イオンを多く含む");
        }
        if is_strong_na_cl && (!chemicals.contains(&NaIon) || !chemicals.contains(&ClIon)) {
            panic!("塩化物強塩泉には必ずナトリウムイオンと塩化物イオンを含む");
        }
        if is_weak_rn && !chemicals.contains(&Rn) {
            panic!("弱放射能泉には必ずラドンを含む");
        }
        if chemicals.is_empty() {
            return Self {
                is_simple: true,
                liquid,
                cations: vec![],
                anions: vec![],
                inclusions: vec![],
                is_strong_na_cl: false,
                is_weak_rn,
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
            is_strong_na_cl,
            is_weak_rn,
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
            } else if let Rn = target {
                // TODO: Chemicalに統合する
                let mut prefix = "";
                if self.is_weak_rn {
                    prefix = "弱";
                }
                prefix.to_owned() + &target.jp()
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
            .map(|v| {
                // TODO: Chemicalに統合する
                let mut suffix = "";
                if self.is_strong_na_cl && *v == ClIon {
                    suffix = "強塩";
                }
                v.jp() + suffix
            })
            .collect::<Vec<String>>()
            .join("・");
        let inclusion_h_ion_excluded = self
            .inclusions
            .iter()
            .filter(|&v| v.clone() != HIon)
            .collect::<Vec<&Chemical>>();
        let inclusion_enumerated_text = inclusion_h_ion_excluded
            .iter()
            .map(|v| {
                // TODO: Chemicalに統合する
                let mut prefix = "";
                if self.is_weak_rn && v == &&Rn {
                    prefix = "弱";
                }
                prefix.to_owned() + &v.jp()
            })
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
    use crate::domain::onsen::onsen_quality::OnsenQuality;
    use crate::domain::onsen::onsen_quality::SpringLiquid::*;

    #[test]
    fn test_tanjun_onsen() {
        let quality = OnsenQuality::new(&vec![], Some(Neutral), false, false);
        assert_eq!(quality.to_string(), "単純温泉");
        let cloned_quality = quality.clone();
        assert_eq!(cloned_quality.to_string(), "単純温泉");
    }

    #[test]
    fn test_tanjun_onsen_if_no_liquid_is_given() {
        let quality = OnsenQuality::new(&vec![], None, false, false);
        assert_eq!(quality.to_string(), "単純温泉");
    }

    #[test]
    fn test_mildly_alkaline_tanjun_onsen() {
        let quality = OnsenQuality::new(&vec![], Some(MildlyAlkaline), false, false);
        assert_eq!(quality.to_string(), "弱アルカリ性単純温泉");
    }

    #[test]
    fn test_alkaline_tanjun_onsen() {
        let quality = OnsenQuality::new(&vec![], Some(Alkaline), false, false);
        assert_eq!(quality.to_string(), "アルカリ性単純温泉");
    }

    #[test]
    fn test_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon], None, false, false);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物泉");
    }

    #[test]
    fn test_strong_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon], None, true, false);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物強塩泉");
    }

    #[test]
    fn test_strong_na_cl_onsen_if_other_anion_is_less() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon, SO4Ion], None, true, false);
        assert_eq!(quality.to_string(), "ナトリウム－塩化物強塩・硫酸塩泉");
    }

    #[test]
    #[should_panic]
    fn test_is_strong_na_cl_without_its_chemicals() {
        OnsenQuality::new(&vec![], None, true, false);
    }

    #[test]
    fn test_na_mg_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, MgIon, ClIon], None, false, false);
        assert_eq!(quality.to_string(), "ナトリウム・マグネシウム－塩化物泉");
    }

    #[test]
    fn test_na_ca_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, CaIon, ClIon], None, false, false);
        assert_eq!(quality.to_string(), "ナトリウム・カルシウム－塩化物泉");
    }

    #[test]
    fn test_ca_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![CaIon, HCO3Ion], None, false, false);
        assert_eq!(quality.to_string(), "カルシウム－炭酸水素塩泉");
    }

    #[test]
    fn test_na_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, HCO3Ion], None, false, false);
        assert_eq!(quality.to_string(), "ナトリウム－炭酸水素塩泉");
    }

    #[test]
    fn test_so4_onsen() {
        let quality = OnsenQuality::new(&vec![SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "硫酸塩泉");
    }

    #[test]
    fn test_mg_so4_onsen() {
        let quality = OnsenQuality::new(&vec![MgIon, SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "マグネシウム－硫酸塩泉");
    }

    #[test]
    fn test_na_so4_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "ナトリウム－硫酸塩泉");
    }

    #[test]
    fn test_ca_so4_onsen() {
        let quality = OnsenQuality::new(&vec![CaIon, SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "カルシウム－硫酸塩泉");
    }

    #[test]
    fn test_co2_onsen() {
        let quality = OnsenQuality::new(&vec![CO2], None, false, false);
        assert_eq!(quality.to_string(), "単純二酸化炭素泉");
    }

    #[test]
    fn test_fe_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(2)], None, false, false);
        assert_eq!(quality.to_string(), "単純鉄泉");
    }

    #[test]
    fn test_fe_hco3_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(2), HCO3Ion], None, false, false);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－炭酸水素塩泉");
    }

    #[test]
    fn test_fe2_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(2), SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－硫酸塩泉");
    }

    #[test]
    fn test_fe3_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(3), SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "含鉄（Ⅲ）－硫酸塩泉");
    }

    #[test]
    fn test_fe_so4_onsen() {
        let quality = OnsenQuality::new(&vec![FeIon(0), SO4Ion], None, false, false);
        assert_eq!(quality.to_string(), "含鉄－硫酸塩泉");
    }

    #[test]
    fn test_al_onsen() {
        let quality = OnsenQuality::new(
            &vec![S, AlIon, FeIon(2), NaIon, CaIon, SO4Ion],
            None,
            false,
            false,
        );
        assert_eq!(
            quality.to_string(),
            "含硫黄・アルミニウム・鉄（Ⅱ）－ナトリウム・カルシウム－硫酸塩泉"
        );
    }

    #[test]
    fn test_cu_onsen() {
        let quality = OnsenQuality::new(
            &vec![HIon, CuIon, FeIon(2), SO4Ion],
            Some(Acidic),
            false,
            false,
        );
        assert_eq!(quality.to_string(), "酸性－含銅・鉄（Ⅱ）－硫酸塩泉");
    }

    #[test]
    fn test_h_onsen() {
        let quality = OnsenQuality::new(&vec![HIon], Some(Acidic), false, false);
        assert_eq!(quality.to_string(), "単純酸性泉");
    }

    #[test]
    #[should_panic]
    fn test_h_onsen_without_acidic_liquid() {
        OnsenQuality::new(&vec![HIon], Some(Neutral), false, false);
    }

    #[test]
    #[should_panic]
    fn test_acidic_liquid_onsen_without_h_ion() {
        OnsenQuality::new(&vec![], Some(Acidic), false, false);
    }

    #[test]
    fn test_h_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon, HIon], Some(Acidic), false, false);
        assert_eq!(quality.to_string(), "酸性－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_h_s_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![NaIon, ClIon, HIon, S], Some(Acidic), false, false);
        // "酸性"はさらに仕切られる
        assert_eq!(quality.to_string(), "酸性－含硫黄－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_i_na_cl_onsen() {
        let quality = OnsenQuality::new(&vec![IIon, NaIon, ClIon], None, false, false);
        assert_eq!(quality.to_string(), "含よう素－ナトリウム－塩化物泉");
    }

    #[test]
    fn test_s_onsen() {
        let quality = OnsenQuality::new(&vec![S], None, false, false);
        assert_eq!(quality.to_string(), "単純硫黄泉");
    }

    #[test]
    fn test_rn_onsen() {
        let quality = OnsenQuality::new(&vec![Rn], None, false, false);
        assert_eq!(quality.to_string(), "単純放射能泉");
    }

    #[test]
    fn test_weak_rn_onsen() {
        let quality = OnsenQuality::new(&vec![Rn], None, false, true);
        assert_eq!(quality.to_string(), "単純弱放射能泉");
    }

    #[test]
    #[should_panic]
    fn test_is_weak_rn_without_its_chemicals() {
        OnsenQuality::new(&vec![], None, false, true);
    }

    #[test]
    fn test_multi_inclusion_onsen() {
        let quality =
            OnsenQuality::new(&vec![S, FeIon(2), NaIon, CaIon, SO4Ion], None, false, false);
        assert_eq!(
            quality.to_string(),
            // "含"は先頭にのみつける
            "含硫黄・鉄（Ⅱ）－ナトリウム・カルシウム－硫酸塩泉"
        );
    }

    #[test]
    fn test_to_string_vec() {
        let quality = OnsenQuality::new(&vec![FeIon(2), NaIon, HCO3Ion], None, false, false);
        assert_eq!(quality.to_string(), "含鉄（Ⅱ）－ナトリウム－炭酸水素塩泉");
        assert_eq!(quality.to_string_vec(), vec!["NaIon", "HCO3Ion", "FeIon"]);
    }
}
