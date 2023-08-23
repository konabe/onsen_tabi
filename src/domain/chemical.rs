use Chemical::*;

#[derive(PartialEq, Clone)]
pub enum Chemical {
    NaIon,
    CaIon,
    MgIon,
    ClIon,
    HCO3Ion,
    SO4Ion,
    CO2,
    FeIon(u8), // 価数
    HIon,
    IIon,
    S,
    Rn,
}

impl Chemical {
    // 陽イオン
    pub fn is_cation(&self) -> bool {
        return vec![NaIon, CaIon, MgIon].contains(self);
    }

    pub fn is_anion(&self) -> bool {
        return vec![ClIon, HCO3Ion, SO4Ion].contains(self);
    }

    pub fn jp(&self) -> String {
        match self {
            NaIon => "ナトリウム".to_string(),
            CaIon => "カルシウム".to_string(),
            MgIon => "マグネシウム".to_string(),
            ClIon => "塩化物".to_string(),
            HCO3Ion => "炭酸水素塩".to_string(),
            SO4Ion => "硫酸塩".to_string(),
            CO2 => "二酸化炭素".to_string(),
            FeIon(_) => "鉄".to_string(),
            HIon => "酸性".to_string(),
            IIon => "ヨウ素".to_string(),
            S => "硫黄".to_string(),
            Rn => "放射能".to_string(),
        }
    }
}

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
