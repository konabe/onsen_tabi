use strum_macros::{Display, EnumString};

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
/// 温泉の液性
pub enum SpringLiquid {
    #[strum(serialize = "acidic")]
    /// 酸性
    Acidic,
    #[strum(serialize = "mildly_acidic")]
    /// 弱酸性
    MildlyAcidic,
    #[strum(serialize = "neutral")]
    /// 中性
    Neutral,
    #[strum(serialize = "mildly_alkaline")]
    /// 弱アルカリ性
    MildlyAlkaline,
    #[strum(serialize = "alkaline")]
    /// アルカリ性
    Alkaline,
}
