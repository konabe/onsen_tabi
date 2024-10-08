use strum_macros::{Display, EnumString};

#[derive(Display, Debug, PartialEq, EnumString, Clone, Default)]
/// 温泉の営業形態
pub enum SpringForm {
    #[default]
    #[strum(serialize = "uchiyu")]
    /// 内湯
    Uchiyu,
    #[strum(serialize = "sotoyu")]
    /// 外湯
    Sotoyu,
}
