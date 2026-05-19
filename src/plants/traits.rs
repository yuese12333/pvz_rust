use serde::{Deserialize, Serialize};

use super::stats::PlantArchetypeStats;

/// 植物特性标签（与玩法系统读取一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PlantTrait {
    /// 低矮：可被越过，不阻挡撑杆跳僵尸。
    LowProfile,
    /// 夜间：夜间关卡可用，白天关卡会睡着。
    Night,
    /// 穿透：攻击可穿透同行多个僵尸。
    Pierce,
    /// 灰烬：伤害若超过僵尸本体剩余血量，无视防具直接击杀。
    Ash,
    /// 减速：命中僵尸时附加减速；参数见 `slow_duration` / `slow_factor`。
    Slow,
    /// 血量分段：血量降至阈值时触发外观变化。
    HealthSegmented,
    /// 胆小：前方近处有僵尸时缩起并停止攻击。
    Scaredy,
    /// 魅惑：被啃食时魅惑该僵尸为我方单位。
    Hypnotize,
    /// 冻结：对作用范围内僵尸施加冻结效果。
    Freeze,
    /// 坑洞：爆炸后在原地留下无法种植的坑洞。
    Crater,
}

/// 校验 `traits` 与 `slow_duration` / `slow_factor` 等字段一致。
pub fn validate_trait_fields(key: &str, stats: &PlantArchetypeStats) {
    let has_slow = stats.traits.contains(&PlantTrait::Slow);

    match (has_slow, stats.slow_duration, stats.slow_factor) {
        (true, Some(duration), Some(factor)) => {
            if !(factor > 0.0 && factor < 1.0) {
                panic!("{key} 的 slow_factor 须在 (0.0, 1.0) 内，当前为 {factor}");
            }
            if duration <= 0.0 {
                panic!("{key} 的 slow_duration 须 > 0");
            }
        }
        (true, _, _) => {
            panic!("{key} 含 Slow 特性时 slow_duration 与 slow_factor 均须为 Some");
        }
        (false, None, None) => {}
        (false, _, _) => {
            panic!("{key} 无 Slow 特性时不得配置 slow_duration / slow_factor");
        }
    }
}
