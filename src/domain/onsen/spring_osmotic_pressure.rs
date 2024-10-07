use strum_macros::{Display, EnumString};

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
/// 温泉の浸透圧
pub enum SpringOsmoticPressure {
    #[strum(serialize = "hypotonic")]
    /// 低張性
    Hypotonic,
    #[strum(serialize = "isotonic")]
    /// 等張性
    Isotonic,
    #[strum(serialize = "hypertonic")]
    /// 高張性
    Hypertonic,
}
