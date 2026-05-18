//! 僵尸数据与通用规则（与 `mechanics_and_values.md`、`assets/data/zombies.ron` 对齐）。

pub mod dancing;
pub mod hp;
pub mod plugin;

pub use hp::apply_dying_drain;
// pub use hp::{ZombieBodyHp, ZombieHpStage};
pub use plugin::ZombiesPlugin;

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use crate::armors::ArmorsCatalog;

/// 僵尸种类（与 `assets/data/zombies.ron` 键名一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZombieType {
    /// #1 普通僵尸。
    Zombie,
    /// #2 旗帜僵尸。
    FlagZombie,
    /// #3 路障僵尸。
    ConeheadZombie,
    /// #4 撑杆跳僵尸。
    PoleVaultingZombie,
    /// #5 铁桶僵尸。
    BucketheadZombie,
    /// #6 读报僵尸。
    NewspaperZombie,
    /// #7 铁门僵尸。
    ScreenDoorZombie,
    /// #8 橄榄球僵尸。
    FootballZombie,
    /// #9 舞王僵尸。
    DancingZombie,
    /// #10 伴舞僵尸。
    BackupDancerZombie,
}

impl ZombieType {
    /// 当前在目录中有独立条目的种类（与 `zombies.ron` 键一致；扩展时在此追加）。
    pub const ALL: [Self; 10] = [
        Self::Zombie,
        Self::FlagZombie,
        Self::ConeheadZombie,
        Self::PoleVaultingZombie,
        Self::BucketheadZombie,
        Self::NewspaperZombie,
        Self::ScreenDoorZombie,
        Self::FootballZombie,
        Self::DancingZombie,
        Self::BackupDancerZombie,
    ];

    /// RON 中的字符串键名。
    #[must_use]
    pub fn ron_key(self) -> &'static str {
        match self {
            Self::Zombie => "Zombie",
            Self::FlagZombie => "FlagZombie",
            Self::ConeheadZombie => "ConeheadZombie",
            Self::PoleVaultingZombie => "PoleVaultingZombie",
            Self::BucketheadZombie => "BucketheadZombie",
            Self::NewspaperZombie => "NewspaperZombie",
            Self::ScreenDoorZombie => "ScreenDoorZombie",
            Self::FootballZombie => "FootballZombie",
            Self::DancingZombie => "DancingZombie",
            Self::BackupDancerZombie => "BackupDancerZombie",
        }
    }

    /// 是否为舞王（含状态机，见机制篇 §3）。
    #[must_use]
    #[allow(dead_code)]
    pub fn is_dancing_king(self) -> bool {
        matches!(self, Self::DancingZombie)
    }

    /// 是否为伴舞（仅由舞王召唤生成）。
    #[must_use]
    #[allow(dead_code)]
    pub fn is_backup_dancer(self) -> bool {
        matches!(self, Self::BackupDancerZombie)
    }

    // /// 是否适用机制篇 §2.2 本体三段血量与垂死（例外种类见文档 §2.2 段首）。
    // #[must_use]
    // pub fn has_segmented_hp(self) -> bool {
    //     match self {
    //         Self::Zombie
    //         | Self::FlagZombie
    //         | Self::ConeheadZombie
    //         | Self::PoleVaultingZombie
    //         | Self::BucketheadZombie => true,
    //         // 扩展后在此追加例外并返回 false，例如：Self::Gargantuar => false,
    //     }
    // }
}

/// 单种僵尸在 RON 中的公共数值（防具为可选；移速为「每格秒数」区间，出场时在区间内随机一次后固定）。
///
/// `score` > 0 且 `weight` > 0 时参与 `max_points` 点数加权池（与 `min_wave` 波次过滤配合）；**旗帜波领队等表现**可在关卡逻辑中另行约束。
///
/// 攻击、帧数、计分等字段由 RON 驱动，玩法系统接入前可能尚未读取。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ZombieArchetypeStats {
    /// 防具键名，对应 `armor.ron` 条目；无防具为 `None`。
    #[serde(default)]
    pub armor: Option<String>,
    /// 本体血量（必有）。
    pub body_hp: f32,
    /// 每格移动时间下限（秒），含端点。
    pub secs_per_cell_min: f64,
    /// 每格移动时间上限（秒），含端点。
    pub secs_per_cell_max: f64,
    /// 状态切换后的每格秒数下限（如撑杆落地、读报狂暴、舞王舞蹈-移动循环）；与 `state_secs_per_cell_max` 同时为 `Some` 时由玩法逻辑读取。
    #[serde(default)]
    pub state_secs_per_cell_min: Option<f64>,
    /// 状态切换后的每格秒数上限。
    #[serde(default)]
    pub state_secs_per_cell_max: Option<f64>,
    pub attack_damage: f32,
    pub attack_interval: f64,
    pub sprite_dir: String,
    pub walk_frames: u32,
    pub eat_frames: u32,
    pub die_frames: u32,
    /// 出怪点数；≤0 时不参与点数预算随机池。
    pub score: i32,
    /// 出怪权重；0 时不参与加权随机。
    pub weight: u32,
    /// 最早可进入点数池的波次（1 = 第一波起）。
    pub min_wave: u32,
}

impl ZombieArchetypeStats {
    /// 是否参与「`max_points` + 权重 + `min_wave`」出怪池。
    #[must_use]
    pub fn participates_in_point_spawn_pool(&self) -> bool {
        self.score > 0 && self.weight > 0
    }
}

/// 校验合并后的僵尸数值（全局或关卡覆盖后均可调用）。
pub fn validate_zombie_archetype(
    ty: ZombieType,
    stats: &ZombieArchetypeStats,
    armors: &ArmorsCatalog,
) {
    validate_zombie_entry(ty, stats, armors);
}

fn validate_zombie_entry(ty: ZombieType, stats: &ZombieArchetypeStats, armors: &ArmorsCatalog) {
    let key = ty.ron_key();
    if stats.body_hp <= 0.0 {
        panic!("{key} 的 body_hp 须 > 0");
    }
    if stats.secs_per_cell_min > stats.secs_per_cell_max {
        panic!(
            "{key} 的 secs_per_cell_min={} 不能大于 secs_per_cell_max={}",
            stats.secs_per_cell_min, stats.secs_per_cell_max
        );
    }
    if stats.score < 0 {
        panic!("{key} 的 score 不能为负数");
    }

    let in_pool = stats.participates_in_point_spawn_pool();
    let script_only = stats.score <= 0 && stats.weight == 0;
    if !in_pool && !script_only {
        panic!(
            "{key} 的 score/weight 不一致：参与点数池须 score>0 且 weight>0；\
             仅脚本/事件生成须 score<=0 且 weight=0（当前 score={}, weight={}）",
            stats.score, stats.weight
        );
    }
    if in_pool && stats.min_wave < 1 {
        panic!("{key} 参与点数池时 min_wave 须 >= 1");
    }
    if let Some(armor_key) = &stats.armor {
        if armor_key.is_empty() {
            panic!("{key} 的 armor 键名不能为空字符串");
        }
        if armors.get_by_key(armor_key).is_none() {
            panic!("{key} 的 armor=\"{armor_key}\" 须在 armor.ron 中存在");
        }
    }
    if ty == ZombieType::DancingZombie {
        let (Some(lo), Some(hi)) = (
            stats.state_secs_per_cell_min,
            stats.state_secs_per_cell_max,
        ) else {
            panic!("{key} 须配置 state_secs_per_cell_*（舞蹈-移动循环移速）");
        };
        if lo > hi {
            panic!(
                "{key} 的 state_secs_per_cell_min={lo} 不能大于 state_secs_per_cell_max={hi}"
            );
        }
    }
}

/// 启动时加载 `assets/data/zombies.ron` 并校验 [`ZombieType::ALL`] 条目齐全且配置合法。
#[derive(Resource, Debug)]
pub struct ZombiesCatalog {
    entries: HashMap<String, ZombieArchetypeStats>,
}

impl ZombiesCatalog {
    /// 从项目根相对路径读取；缺少任一种类或配置非法时 panic。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str, armors: &ArmorsCatalog) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取僵尸配置 {full:?}: {e}"));
        let entries: HashMap<String, ZombieArchetypeStats> = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析僵尸配置 {full:?}: {e}"));
        if entries.len() != ZombieType::ALL.len() {
            let keys: Vec<_> = entries.keys().cloned().collect();
            panic!(
                "zombies.ron 条目数须与 ZombieType::ALL 一致（{}），实际 {} 条: {keys:?}",
                ZombieType::ALL.len(),
                entries.len()
            );
        }
        for ty in ZombieType::ALL {
            let key = ty.ron_key();
            let stats = entries
                .get(key)
                .unwrap_or_else(|| panic!("zombies.ron 须包含条目: {key}"));
            validate_zombie_entry(ty, stats, armors);
        }
        Self { entries }
    }

    /// 按种类取配置。
    #[must_use]
    pub fn get(&self, ty: ZombieType) -> Option<&ZombieArchetypeStats> {
        self.entries.get(ty.ron_key())
    }

    // #[must_use]
    // pub fn len(&self) -> usize {
    //     self.entries.len()
    // }
}
