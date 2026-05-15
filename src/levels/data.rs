//! 关卡 RON 反序列化（与 `assets/data/levels/*.ron` 对齐）。

use serde::Deserialize;

/// 根关卡文件（如 `level_1.ron`）。字段由关卡、选卡、波次等系统读取，当前部分仅在 `LevelsPlugin` 冒烟断言中访问。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LevelDef {
    pub background: String,
    pub bgm: String,
    pub initial_sun: u32,
    pub plant_slots: Vec<String>,
    pub waves: Vec<WaveDef>,
    pub victory_condition: VictoryCondition,
}

/// 单波配置：`max_points` 为出怪点数预算（与 `zombies.ron` 中 `score`、`weight`、`min_wave` 协同）；具体僵尸由波次调度按预算与权重生成，不在关卡 RON 里硬编码。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct WaveDef {
    pub trigger: WaveTrigger,
    pub max_points: i32,
    #[serde(default)]
    pub is_final: bool,
}

/// 波次触发条件。
#[derive(Debug, Clone, Deserialize)]
pub enum WaveTrigger {
    Time(f32),
    AllDead,
}

/// 胜利条件（占位扩展）。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum VictoryCondition {
    AllWavesCleared,
}
