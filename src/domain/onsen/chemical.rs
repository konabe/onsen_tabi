use strum_macros::{Display, EnumString};
use Chemical::*;

#[allow(dead_code)]
#[derive(Display, PartialEq, Clone, EnumString, Debug)]
pub enum Chemical {
    NaIon,
    CaIon,
    MgIon,
    ClIon,
    HCO3Ion,
    SO4Ion,
    CO2,
    FeIon(u8), // 価数
    AlIon,
    CuIon,
    HIon,
    IIon,
    S,
    Rn,
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
            ClIon | HCO3Ion | SO4Ion => true,
            _ => false,
        }
    }

    pub fn is_inclusion(&self) -> bool {
        match self {
            CO2 | FeIon(_) | AlIon | CuIon | HIon | IIon | S | Rn => true,
            _ => false,
        }
    }

    pub fn jp(&self) -> String {
        let str = match self {
            NaIon => "ナトリウム",
            CaIon => "カルシウム",
            MgIon => "マグネシウム",
            ClIon => "塩化物",
            HCO3Ion => "炭酸水素塩",
            SO4Ion => "硫酸塩",
            CO2 => "二酸化炭素",
            FeIon(valence) => match valence {
                2 => "鉄（Ⅱ）",
                3 => "鉄（Ⅲ）",
                _ => "鉄",
            },
            AlIon => "アルミニウム",
            CuIon => "銅",
            HIon => "酸性",
            IIon => "よう素",
            S => "硫黄",
            Rn => "放射能",
        };
        str.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::onsen::chemical::Chemical::*;

    #[test]
    fn test_is_cation() {
        assert!(NaIon.is_cation());
        assert!(CaIon.is_cation());
        assert!(MgIon.is_cation());
        assert!(!ClIon.is_cation());
    }

    #[test]
    fn test_is_anion() {
        assert!(ClIon.is_anion());
        assert!(HCO3Ion.is_anion());
        assert!(SO4Ion.is_anion());
        assert!(!NaIon.is_anion());
    }
}
