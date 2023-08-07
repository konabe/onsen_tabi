use strum_macros::{Display, EnumString};

/// 液性
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum SpringLiquid {
    #[strum(serialize = "acidic")]
    Acidic, // 酸性
    #[strum(serialize = "mildly_acidic")]
    MildlyAcidic, // 弱酸性
    #[strum(serialize = "neutral")]
    Neutral, // 中性
    #[strum(serialize = "mildly_alkaline")]
    MildlyAlkaline, // 弱アルカリ性
    #[strum(serialize = "alkaline")]
    Alkaline, // アルカリ性
}

/// 浸透圧
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum SpringOsmoticPressure {
    #[strum(serialize = "hypotonic")]
    Hypotonic, // 低張性
    #[strum(serialize = "isotonic")]
    Isotonic, // 等張性
    #[strum(serialize = "hypertonic")]
    Hypertonic, // 高張性
}

/// 営業形態
#[derive(Display, Debug, PartialEq, EnumString)]
pub enum SpringForm {
    #[strum(serialize = "uchiyu")]
    Uchiyu, // 内湯
    #[strum(serialize = "sotoyu")]
    Sotoyu, // 外湯
}
