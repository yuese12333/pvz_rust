//! 防具数据层（与 `assets/data/armor.ron` 对齐）。

pub mod plugin;

pub use plugin::ArmorsPlugin;

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

/// 防具种类（与 `assets/data/armor.ron` 键名一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmorType {
    /// 路障（一类）。
    Conehead,
    /// 铁桶（一类）。
    Buckethead,
    /// 报纸（二类）。
    Newspaper,
    /// 铁门（二类）。
    ScreenDoor,
    /// 橄榄球头盔（一类）。
    FootballHelmet,
}

impl ArmorType {
    /// 当前在目录中有独立条目的种类（与 `armor.ron` 键一致；扩展时在此追加）。
    pub const ALL: [Self; 5] = [
        Self::Conehead,
        Self::Buckethead,
        Self::Newspaper,
        Self::ScreenDoor,
        Self::FootballHelmet,
    ];

    /// RON 中的字符串键名。
    #[must_use]
    pub fn ron_key(self) -> &'static str {
        match self {
            Self::Conehead => "Conehead",
            Self::Buckethead => "Buckethead",
            Self::Newspaper => "Newspaper",
            Self::ScreenDoor => "ScreenDoor",
            Self::FootballHelmet => "FootballHelmet",
        }
    }
}

/// 防具层级（见 `mechanics_and_values.md` 三、机制篇 §2.1）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ArmorTier {
    /// 一类（Headwear）：所有攻击必须先打穿。
    Tier1,
    /// 二类（Shield）：抛物线、烟雾、地刺可绕过。
    Tier2,
}

/// 单种防具在 `armor.ron` 中的条目。
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ArmorArchetypeStats {
    pub tier: ArmorTier,
    pub hp: f32,
    pub sprite_dir: String,
}

/// 关卡对单种防具数值的局部覆盖（仅允许覆盖 `hp`）。
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArmorArchetypeOverride {
    #[serde(default)]
    pub hp: Option<f32>,
}

impl ArmorArchetypeOverride {
    /// 将本关覆盖应用到全局基底，得到本关有效数值。
    #[must_use]
    pub fn apply_to(&self, base: &ArmorArchetypeStats) -> ArmorArchetypeStats {
        let mut s = base.clone();
        if let Some(v) = self.hp {
            s.hp = v;
        }
        s
    }
}

/// 校验合并后的防具数值（全局或关卡覆盖后均可调用）。
pub fn validate_armor_archetype(ty: ArmorType, stats: &ArmorArchetypeStats) {
    validate_armor_entry(ty, stats);
}

fn validate_armor_entry(ty: ArmorType, stats: &ArmorArchetypeStats) {
    let key = ty.ron_key();
    if stats.hp <= 0.0 {
        panic!("{key} 的 hp 须 > 0");
    }
    if stats.sprite_dir.is_empty() {
        panic!("{key} 的 sprite_dir 不能为空");
    }
}

/// 启动时加载 `assets/data/armor.ron` 并校验 [`ArmorType::ALL`] 条目齐全且配置合法。
#[derive(Resource, Debug)]
pub struct ArmorsCatalog {
    entries: HashMap<String, ArmorArchetypeStats>,
}

impl ArmorsCatalog {
    /// 从项目根相对路径读取；缺少任一种类或配置非法时 panic。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取防具配置 {full:?}: {e}"));
        let entries: HashMap<String, ArmorArchetypeStats> = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析防具配置 {full:?}: {e}"));
        if entries.len() != ArmorType::ALL.len() {
            let keys: Vec<_> = entries.keys().cloned().collect();
            panic!(
                "armor.ron 条目数须与 ArmorType::ALL 一致（{}），实际 {} 条: {keys:?}",
                ArmorType::ALL.len(),
                entries.len()
            );
        }
        for ty in ArmorType::ALL {
            let key = ty.ron_key();
            let stats = entries
                .get(key)
                .unwrap_or_else(|| panic!("armor.ron 须包含条目: {key}"));
            validate_armor_entry(ty, stats);
        }
        Self { entries }
    }

    /// 按枚举取配置。
    #[must_use]
    pub fn get(&self, ty: ArmorType) -> Option<&ArmorArchetypeStats> {
        self.entries.get(ty.ron_key())
    }

    /// 按 RON 键名取配置（如僵尸 `armor` 字段引用的 `"Conehead"`）。
    #[must_use]
    pub fn get_by_key(&self, key: &str) -> Option<&ArmorArchetypeStats> {
        self.entries.get(key)
    }
}
