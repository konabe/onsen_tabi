use strum_macros::{Display, EnumString};

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
/// 温泉の温度を分類する
pub enum SpringTemperature {
    #[strum(serialize = "hot")]
    /// 高温泉
    Hot,
    #[strum(serialize = "normal")]
    /// 温泉
    Warm,
    #[strum(serialize = "cool")]
    /// 低温泉
    Cool,
    #[strum(serialize = "cold")]
    /// 冷鉱泉
    Cold,
}
