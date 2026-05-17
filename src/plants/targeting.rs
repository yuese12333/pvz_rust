//! 植物索敌范围（与 `assets/data/plants.ron` 的 `targeting` 字段对齐）。

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 植物索敌方式；无此字段的植物不参与索敌。
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum PlantTargeting {
    /// 前方一整行（射手类）。
    LaneForward,
    /// 前方 N 格内（小喷菇、大嘴花等）。
    ForwardRange(f32),
    /// 本行双向 N 格内（土豆雷等）。
    RowRadius(f32),
    /// 3×3 范围（樱桃炸弹等）。
    Area3x3,
    /// 全屏。
    Global,
}

/// 字段存在时反序列化为 `Some(...)`；字段省略由 `#[serde(default)]` 得到 `None`，不会调用本函数。
pub fn deserialize_present<'de, D>(deserializer: D) -> Result<Option<PlantTargeting>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(PlantTargeting::deserialize(deserializer)?))
}

/// 序列化时省略 `Some` 包装，与 RON 习惯一致。
pub fn serialize_optional<S>(
    value: &Option<PlantTargeting>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(t) => t.serialize(serializer),
    }
}
