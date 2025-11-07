use strum_macros::{Display, EnumString};
use Chemical::*;

#[derive(Display, PartialEq, Clone, Copy, Debug, Default)]
/// 塩化物イオンタイプ
pub enum ClType {
    #[default]
    /// 通常
    Normal,
    /// 強塩
    Strong,
}

impl ClType {
    pub fn jp(&self) -> &str {
        match self {
            ClType::Normal => "塩化物",
            ClType::Strong => "塩化物強塩",
        }
    }
}

#[derive(Display, PartialEq, Clone, Copy, Debug, Default)]
/// 鉄イオンタイプ
pub enum FeType {
    #[default]
    /// 通常
    Normal,
    /// Ⅱ価
    Two,
    /// Ⅲ価
    Three,
}

impl FeType {
    /// 日本語表記
    pub fn jp(&self) -> &str {
        match self {
            FeType::Normal => "鉄",
            FeType::Two => "鉄（Ⅱ）",
            FeType::Three => "鉄（Ⅲ）",
        }
    }
}

#[derive(Display, PartialEq, Clone, Copy, Debug, Default)]
/// ラドンタイプ
pub enum RnType {
    #[default]
    /// 放射能
    Normal,
    /// 弱放射能
    Weak,
}

impl RnType {
    /// 日本語表記
    pub fn jp(&self) -> &str {
        match self {
            RnType::Normal => "放射能",
            RnType::Weak => "弱放射能",
        }
    }
}

#[allow(dead_code)]
#[derive(Display, PartialEq, Clone, Copy, EnumString, Debug)]
/// 温泉成分
pub enum Chemical {
    /// ナトリウムイオン
    NaIon,
    /// カルシウムイオン
    CaIon,
    /// マグネシウムイオン
    MgIon,
    /// 塩化物イオン
    ClIon(ClType),
    /// 炭酸水素イオン
    HCO3Ion,
    /// 硫酸塩イオン
    SO4Ion,
    /// 二酸化炭素
    CO2,
    /// 鉄イオン
    FeIon(FeType), // 価数
    /// アルミニウムイオン
    AlIon,
    /// 銅イオン
    CuIon,
    /// 水素イオン
    HIon,
    /// よう素
    IIon,
    /// 硫黄
    S,
    /// ラドン
    Rn(RnType),
}

impl Chemical {
    /// 陽イオンであるかどうか
    pub fn is_cation(&self) -> bool {
        matches!(self, NaIon | CaIon | MgIon)
    }

    /// 陰イオンであるかどうか
    pub fn is_anion(&self) -> bool {
        matches!(self, ClIon(_) | HCO3Ion | SO4Ion)
    }

    /// 含有成分であるかどうか
    pub fn is_inclusion(&self) -> bool {
        matches!(
            self,
            CO2 | FeIon(_) | AlIon | CuIon | HIon | IIon | S | Rn(_)
        )
    }

    /// 日本語表記
    pub fn jp(&self) -> &str {
        match self {
            NaIon => "ナトリウム",
            CaIon => "カルシウム",
            MgIon => "マグネシウム",
            ClIon(cl_type) => cl_type.jp(),
            HCO3Ion => "炭酸水素塩",
            SO4Ion => "硫酸塩",
            CO2 => "二酸化炭素",
            FeIon(valence) => valence.jp(),
            AlIon => "アルミニウム",
            CuIon => "銅",
            HIon => "酸性",
            IIon => "よう素",
            S => "硫黄",
            Rn(rn_type) => rn_type.jp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::onsen::chemical::{Chemical::*, ClType, FeType, RnType};

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

    #[test]
    fn test_is_inclusion() {
        assert!(CO2.is_inclusion());
        assert!(FeIon(FeType::Normal).is_inclusion());
        assert!(AlIon.is_inclusion());
        assert!(CuIon.is_inclusion());
        assert!(HIon.is_inclusion());
        assert!(S.is_inclusion());
        assert!(Rn(RnType::Normal).is_inclusion());
        assert!(!NaIon.is_inclusion());
    }

    #[test]
    fn test_jp() {
        assert_eq!(NaIon.jp(), "ナトリウム");
        assert_eq!(CaIon.jp(), "カルシウム");
        assert_eq!(MgIon.jp(), "マグネシウム");
        assert_eq!(ClIon(ClType::Normal).jp(), "塩化物");
        assert_eq!(ClIon(ClType::Strong).jp(), "塩化物強塩");
        assert_eq!(HCO3Ion.jp(), "炭酸水素塩");
        assert_eq!(SO4Ion.jp(), "硫酸塩");
        assert_eq!(CO2.jp(), "二酸化炭素");
        assert_eq!(FeIon(FeType::Normal).jp(), "鉄");
        assert_eq!(FeIon(FeType::Two).jp(), "鉄（Ⅱ）");
        assert_eq!(FeIon(FeType::Three).jp(), "鉄（Ⅲ）");
        assert_eq!(AlIon.jp(), "アルミニウム");
        assert_eq!(CuIon.jp(), "銅");
        assert_eq!(HIon.jp(), "酸性");
        assert_eq!(IIon.jp(), "よう素");
        assert_eq!(S.jp(), "硫黄");
        assert_eq!(Rn(RnType::Normal).jp(), "放射能");
        assert_eq!(Rn(RnType::Weak).jp(), "弱放射能");
    }
}
