use strum_macros::{Display, EnumString};
use Chemical::*;

#[derive(Display, PartialEq, Clone, Debug, Default)]
pub enum ClType {
    #[default]
    Normal,
    Strong,
}

#[derive(Display, PartialEq, Clone, Debug, Default)]
pub enum FeType {
    #[default]
    Normal,
    Two,
    Three,
}

#[derive(Display, PartialEq, Clone, Debug, Default)]
pub enum RnType {
    #[default]
    Normal,
    Weak,
}

#[allow(dead_code)]
#[derive(Display, PartialEq, Clone, EnumString, Debug)]
pub enum Chemical {
    NaIon,
    CaIon,
    MgIon,
    ClIon(ClType),
    HCO3Ion,
    SO4Ion,
    CO2,
    FeIon(FeType), // 価数
    AlIon,
    CuIon,
    HIon,
    IIon,
    S,
    Rn(RnType),
}

impl Chemical {
    // 陽イオン
    pub fn is_cation(&self) -> bool {
        match self {
            NaIon | CaIon | MgIon => true,
            _ => false,
        }
    }

    pub fn is_anion(&self) -> bool {
        match self {
            ClIon(_) | HCO3Ion | SO4Ion => true,
            _ => false,
        }
    }

    pub fn is_inclusion(&self) -> bool {
        match self {
            CO2 | FeIon(_) | AlIon | CuIon | HIon | IIon | S | Rn(_) => true,
            _ => false,
        }
    }

    pub fn jp(&self) -> String {
        let str = match self {
            NaIon => "ナトリウム",
            CaIon => "カルシウム",
            MgIon => "マグネシウム",
            ClIon(cl_type) => match cl_type {
                ClType::Normal => "塩化物",
                ClType::Strong => "塩化物強塩",
            },
            HCO3Ion => "炭酸水素塩",
            SO4Ion => "硫酸塩",
            CO2 => "二酸化炭素",
            FeIon(valence) => match valence {
                FeType::Normal => "鉄",
                FeType::Two => "鉄（Ⅱ）",
                FeType::Three => "鉄（Ⅲ）",
            },
            AlIon => "アルミニウム",
            CuIon => "銅",
            HIon => "酸性",
            IIon => "よう素",
            S => "硫黄",
            Rn(rn_type) => match rn_type {
                RnType::Normal => "放射能",
                RnType::Weak => "弱放射能",
            },
        };
        str.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::onsen::chemical::{Chemical::*, ClType};

    #[test]
    fn test_is_cation() {
        assert!(NaIon.is_cation());
        assert!(CaIon.is_cation());
        assert!(MgIon.is_cation());
        assert!(!ClIon(ClType::Normal).is_cation());
    }

    #[test]
    fn test_is_anion() {
        assert!(ClIon(ClType::Normal).is_anion());
        assert!(HCO3Ion.is_anion());
        assert!(SO4Ion.is_anion());
        assert!(!NaIon.is_anion());
    }
}
