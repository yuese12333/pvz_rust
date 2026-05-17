//! 僵尸数据与通用规则（与 `mechanics_and_values.md`、`assets/data/zombies.ron` 对齐）。

pub mod hp;
pub mod plugin;

#[allow(unused_imports)]
pub use hp::{apply_dying_drain, update_stage_after_hp_change, ZombieBodyHp, ZombieHpStage};
pub use plugin::ZombiesPlugin;

use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;

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
}

impl ZombieType {
    /// 当前在目录中有独立条目的种类（与 `zombies.ron` 键一致；扩展时在此追加）。
    pub const ALL: [Self; 4] = [
        Self::Zombie,
        Self::FlagZombie,
        Self::ConeheadZombie,
        Self::PoleVaultingZombie,
    ];

    /// RON 中的字符串键名。
    #[must_use]
    pub fn ron_key(self) -> &'static str {
        match self {
            Self::Zombie => "Zombie",
            Self::FlagZombie => "FlagZombie",
            Self::ConeheadZombie => "ConeheadZombie",
            Self::PoleVaultingZombie => "PoleVaultingZombie",
        }
    }

    /// 是否适用机制篇 §2.2 本体三段血量与垂死（例外种类见文档 §2.2 段首）。
    #[must_use]
    pub fn has_segmented_hp(self) -> bool {
        match self {
            Self::Zombie | Self::FlagZombie | Self::ConeheadZombie | Self::PoleVaultingZombie => true,
            // 扩展后在此追加例外并返回 false，例如：Self::Gargantuar => false,
        }
    }
}

/// 单种僵尸在 RON 中的公共数值（防具为可选；移速为「每格秒数」区间，出场时在区间内随机一次后固定）。
///
/// `score` > 0 且 `weight` > 0 时参与 `max_points` 点数加权池（与 `min_wave` 波次过滤配合）；**旗帜波领队等表现**可在关卡逻辑中另行约束。
///
/// 攻击、帧数、计分等字段由 RON 驱动，玩法系统接入前可能尚未读取。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ZombieArchetypeStats {
    /// 二类防具（Shield，持握式）血量；无则不序列化。
    #[serde(default)]
    pub tier2_armor_hp: Option<f32>,
    /// 一类防具（Headwear，头戴式）血量。
    #[serde(default)]
    pub tier1_armor_hp: Option<f32>,
    /// 本体血量（必有）。
    pub body_hp: f32,
    /// 每格移动时间下限（秒），含端点。
    pub secs_per_cell_min: f64,
    /// 每格移动时间上限（秒），含端点。
    pub secs_per_cell_max: f64,
    /// 失去撑杆后的每格秒数下限；与 `post_vault_secs_per_cell_max` 同时为 `Some` 时，由 [`Self::roll_post_vault_secs_per_cell`] 使用（如撑杆跳僵尸落地后与普通僵尸同速区间）。
    #[serde(default)]
    pub post_vault_secs_per_cell_min: Option<f64>,
    /// 失去撑杆后的每格秒数上限。
    #[serde(default)]
    pub post_vault_secs_per_cell_max: Option<f64>,
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

    /// 按文档规则生成生命值展示串：二类 + 一类 + 本体；无则省略对应段。
    #[must_use]
    pub fn hp_display_string(&self) -> String {
        format_zombie_hp_display(
            self.tier2_armor_hp,
            self.tier1_armor_hp,
            self.body_hp,
        )
    }

    /// 在 `[secs_per_cell_min, secs_per_cell_max]` 内随机每格耗时（秒），出场调用一次后写入实体并固定。
    #[must_use]
    pub fn roll_secs_per_cell<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let lo = self.secs_per_cell_min;
        let hi = self.secs_per_cell_max;
        if lo > hi {
            panic!(
                "僵尸每格秒数配置无效: secs_per_cell_min={lo} > secs_per_cell_max={hi}（sprite_dir={})",
                self.sprite_dir
            );
        }
        rng.gen_range(lo..=hi)
    }

    /// 失去撑杆后每格耗时（秒），在 `[post_vault_secs_per_cell_min, post_vault_secs_per_cell_max]` 内随机；未配置时返回 `None`。
    #[must_use]
    pub fn roll_post_vault_secs_per_cell<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<f64> {
        let (Some(lo), Some(hi)) = (
            self.post_vault_secs_per_cell_min,
            self.post_vault_secs_per_cell_max,
        ) else {
            return None;
        };
        if lo > hi {
            panic!(
                "post_vault 每格秒数无效: min={lo} > max={hi}（sprite_dir={})",
                self.sprite_dir
            );
        }
        Some(rng.gen_range(lo..=hi))
    }
}

/// 生命值展示：二类 + 一类 + 本体；仅有本体时只写 `270` 等形式。
#[must_use]
pub fn format_zombie_hp_display(
    tier2_armor_hp: Option<f32>,
    tier1_armor_hp: Option<f32>,
    body_hp: f32,
) -> String {
    let mut out = String::new();
    if let Some(a) = tier2_armor_hp {
        if !out.is_empty() {
            out.push('+');
        }
        out.push_str(&format!("{}（二类）", fmt_hp_component(a)));
    }
    if let Some(b) = tier1_armor_hp {
        if !out.is_empty() {
            out.push('+');
        }
        out.push_str(&format!("{}（一类）", fmt_hp_component(b)));
    }
    if !out.is_empty() {
        out.push('+');
    }
    out.push_str(&fmt_hp_component(body_hp));
    out
}

fn fmt_hp_component(v: f32) -> String {
    if (v.fract()).abs() < 1e-4 {
        format!("{}", v.round() as i32)
    } else {
        format!("{v}")
    }
}

/// 校验合并后的僵尸数值（全局或关卡覆盖后均可调用）。
pub fn validate_zombie_archetype(ty: ZombieType, stats: &ZombieArchetypeStats) {
    validate_zombie_entry(ty, stats);
}

fn validate_zombie_entry(ty: ZombieType, stats: &ZombieArchetypeStats) {
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
}

/// 启动时加载 `assets/data/zombies.ron` 并校验 [`ZombieType::ALL`] 条目齐全且配置合法。
#[derive(Resource, Debug)]
pub struct ZombiesCatalog {
    entries: HashMap<String, ZombieArchetypeStats>,
}

impl ZombiesCatalog {
    /// 从项目根相对路径读取；缺少任一种类或配置非法时 panic。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
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
            validate_zombie_entry(ty, stats);
        }
        Self { entries }
    }

    /// 按种类取配置。
    #[must_use]
    pub fn get(&self, ty: ZombieType) -> Option<&ZombieArchetypeStats> {
        self.entries.get(ty.ron_key())
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hp_display_body_only() {
        assert_eq!(
            format_zombie_hp_display(None, None, 270.0),
            "270"
        );
    }

    #[test]
    fn hp_display_tier1_plus_body() {
        assert_eq!(
            format_zombie_hp_display(None, Some(200.0), 270.0),
            "200（一类）+270"
        );
    }

    #[test]
    fn hp_display_tier2_plus_body() {
        assert_eq!(
            format_zombie_hp_display(Some(300.0), None, 270.0),
            "300（二类）+270"
        );
    }

    #[test]
    fn hp_display_all_three() {
        assert_eq!(
            format_zombie_hp_display(Some(100.0), Some(200.0), 270.0),
            "100（二类）+200（一类）+270"
        );
    }

    #[test]
    fn script_only_spawn_fields_pass_validation() {
        let stats = ZombieArchetypeStats {
            tier2_armor_hp: None,
            tier1_armor_hp: None,
            body_hp: 270.0,
            secs_per_cell_min: 3.7,
            secs_per_cell_max: 3.7,
            post_vault_secs_per_cell_min: None,
            post_vault_secs_per_cell_max: None,
            attack_damage: 100.0,
            attack_interval: 1.0,
            sprite_dir: "textures/zombies/flag".to_string(),
            walk_frames: 1,
            eat_frames: 1,
            die_frames: 1,
            score: 0,
            weight: 0,
            min_wave: 1,
        };
        validate_zombie_archetype(ZombieType::FlagZombie, &stats);
        assert!(!stats.participates_in_point_spawn_pool());
    }

    #[test]
    #[should_panic(expected = "score/weight 不一致")]
    fn ambiguous_spawn_fields_fail_validation() {
        let stats = ZombieArchetypeStats {
            tier2_armor_hp: None,
            tier1_armor_hp: None,
            body_hp: 270.0,
            secs_per_cell_min: 4.1,
            secs_per_cell_max: 5.3,
            post_vault_secs_per_cell_min: None,
            post_vault_secs_per_cell_max: None,
            attack_damage: 100.0,
            attack_interval: 1.0,
            sprite_dir: String::new(),
            walk_frames: 1,
            eat_frames: 1,
            die_frames: 1,
            score: 100,
            weight: 0,
            min_wave: 1,
        };
        validate_zombie_archetype(ZombieType::Zombie, &stats);
    }

    #[test]
    fn roll_secs_per_cell_stays_in_closed_range() {
        let stats = ZombieArchetypeStats {
            tier2_armor_hp: None,
            tier1_armor_hp: None,
            body_hp: 270.0,
            secs_per_cell_min: 4.1,
            secs_per_cell_max: 5.3,
            attack_damage: 0.0,
            attack_interval: 1.0,
            sprite_dir: String::new(),
            walk_frames: 0,
            eat_frames: 0,
            die_frames: 0,
            score: 1,
            weight: 1,
            min_wave: 1,
            post_vault_secs_per_cell_min: None,
            post_vault_secs_per_cell_max: None,
        };
        let mut rng = rand::thread_rng();
        for _ in 0..64 {
            let v = stats.roll_secs_per_cell(&mut rng);
            assert!((4.1..=5.3).contains(&v), "got {v}");
        }
    }
}
