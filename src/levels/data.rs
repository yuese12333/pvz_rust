//! 关卡 RON 反序列化（与 `assets/data/levels/*.ron` 对齐）。

use std::collections::HashMap;

use serde::Deserialize;

use crate::armors::ArmorArchetypeOverride;
use crate::plants::PlantArchetypeOverride;
use crate::zombies::{ZombieArchetypeStats, ZombieType};

/// 根关卡文件（如 `level_1_1.ron`）。字段由关卡、选卡、波次等系统读取。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LevelDef {
    pub background: String,
    pub bgm: String,
    pub initial_sun: u32,
    pub plant_slots: Vec<String>,
    pub waves: Vec<WaveDef>,
    pub victory_condition: VictoryCondition,
    /// 本关允许进入点数随机池的僵尸种类；缺省或省略时表示使用 `zombies.ron` 中全部已配置种类。
    #[serde(default)]
    pub zombie_pool: Option<Vec<String>>,
    /// 仅在本关覆盖 `zombies.ron` 中对应种类的字段；未写的键保持全局默认。
    #[serde(default)]
    pub zombie_overrides: Option<HashMap<String, ZombieArchetypeOverride>>,
    /// 仅在本关覆盖 `plants.ron` 中对应种类的字段；未写的键保持全局默认。
    #[serde(default)]
    pub plant_overrides: Option<HashMap<String, PlantArchetypeOverride>>,
    /// 仅在本关覆盖 `armor.ron` 中对应防具的 `hp`；未写的键保持全局默认。
    #[serde(default)]
    pub armor_overrides: Option<HashMap<String, ArmorArchetypeOverride>>,
}

impl LevelDef {
    /// 该种类是否允许在本关参与出怪（受 `zombie_pool` 约束）。
    #[must_use]
    pub fn allows_spawn_kind(&self, ty: ZombieType) -> bool {
        match &self.zombie_pool {
            None => true,
            Some(pool) => pool.iter().any(|k| k == ty.ron_key()),
        }
    }
}

/// 关卡对单种僵尸数值的局部覆盖（RON 中只写需要改的字段，与 [`ZombieArchetypeStats`] 同名）。
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ZombieArchetypeOverride {
    #[serde(default)]
    pub armor: Option<String>,
    #[serde(default)]
    pub body_hp: Option<f32>,
    #[serde(default)]
    pub secs_per_cell_min: Option<f64>,
    #[serde(default)]
    pub secs_per_cell_max: Option<f64>,
    #[serde(default)]
    pub state_secs_per_cell_min: Option<f64>,
    #[serde(default)]
    pub state_secs_per_cell_max: Option<f64>,
    #[serde(default)]
    pub attack_damage: Option<f32>,
    #[serde(default)]
    pub attack_interval: Option<f64>,
    #[serde(default)]
    pub sprite_dir: Option<String>,
    #[serde(default)]
    pub walk_frames: Option<u32>,
    #[serde(default)]
    pub eat_frames: Option<u32>,
    #[serde(default)]
    pub die_frames: Option<u32>,
    #[serde(default)]
    pub score: Option<i32>,
    #[serde(default)]
    pub weight: Option<u32>,
    #[serde(default)]
    pub min_wave: Option<u32>,
}

impl ZombieArchetypeOverride {
    /// 将本关覆盖应用到全局基底，得到本关有效数值。
    #[must_use]
    pub fn apply_to(&self, base: &ZombieArchetypeStats) -> ZombieArchetypeStats {
        let mut s = base.clone();
        if let Some(ref v) = self.armor {
            s.armor = Some(v.clone());
        }
        macro_rules! set {
            ($field:ident) => {
                if let Some(v) = self.$field {
                    s.$field = v;
                }
            };
        }
        set!(body_hp);
        set!(secs_per_cell_min);
        set!(secs_per_cell_max);
        if let Some(v) = self.state_secs_per_cell_min {
            s.state_secs_per_cell_min = Some(v);
        }
        if let Some(v) = self.state_secs_per_cell_max {
            s.state_secs_per_cell_max = Some(v);
        }
        set!(attack_damage);
        set!(attack_interval);
        if let Some(ref v) = self.sprite_dir {
            s.sprite_dir = v.clone();
        }
        set!(walk_frames);
        set!(eat_frames);
        set!(die_frames);
        set!(score);
        set!(weight);
        set!(min_wave);
        s
    }
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
    #[allow(dead_code)]
    Time(f32),
    AllDead,
}

/// 胜利条件（占位扩展）。
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum VictoryCondition {
    AllWavesCleared,
}
