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
        return vec![Self::NaIon, Self::CaIon, Self::MgIon].contains(self);
    }

    pub fn is_anion(&self) -> bool {
        return vec![Self::ClIon, Self::HCO3Ion, Self::SO4Ion].contains(self);
    }

    pub fn jp(&self) -> String {
        match self {
            Self::NaIon => "ナトリウム".to_string(),
            Self::CaIon => "カルシウム".to_string(),
            Self::MgIon => "マグネシウム".to_string(),
            Self::ClIon => "塩化物".to_string(),
            Self::HCO3Ion => "炭酸水素".to_string(),
            Self::SO4Ion => "硫酸".to_string(),
            Self::CO2 => "二酸化炭素".to_string(),
            Self::FeIon(_) => "鉄".to_string(),
            Self::HIon => "酸性".to_string(),
            Self::IIon => "ヨウ素".to_string(),
            Self::S => "硫黄".to_string(),
            Self::Rn => "放射能".to_string(),
        }
    }
}

#[test]
fn test_is_cation() {
    assert!(Chemical::NaIon.is_cation());
    assert!(Chemical::CaIon.is_cation());
    assert!(Chemical::MgIon.is_cation());
    assert!(!Chemical::ClIon.is_cation());
}

#[test]
fn test_is_anion() {
    assert!(Chemical::ClIon.is_anion());
    assert!(Chemical::HCO3Ion.is_anion());
    assert!(Chemical::SO4Ion.is_anion());
    assert!(!Chemical::NaIon.is_anion());
}
