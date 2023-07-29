/// 液性
pub enum SpringLiquid {
    Acidic,         // 酸性
    MildlyAcidic,   // 弱酸性
    Neutral,        // 中性
    MildlyAlkaline, // 弱アルカリ性
    Alkaline,       // アルカリ性
}

/// 浸透圧
pub enum SpringOsmoticPressure {
    Hypotonic,  // 低張性
    Isotonic,   // 等張性
    Hypertonic, // 高張性
}

/// 営業形態
pub enum SpringForm {
    Uchiyu, // 内湯
    Sotoyu, // 外湯
}
