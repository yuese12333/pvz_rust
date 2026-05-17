//! 植物数据层：与 `mechanics_and_values.md` 及 `assets/data/plants.ron` 对齐。

pub mod plugin;
pub mod stats;
pub mod targeting;

pub use plugin::PlantsPlugin;
pub use stats::{validate_plant_archetype, PlantArchetypeOverride, PlantArchetypeStats};
pub use targeting::PlantTargeting;

use std::collections::HashMap;

use bevy::prelude::*;
use stats::validate_plant_entry;

/// 植物类型（与 `assets/data/plants.ron` 键名一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantType {
    /// #1 豌豆射手。
    Peashooter,
    /// #2 向日葵。
    Sunflower,
    /// #3 樱桃炸弹。
    CherryBomb,
    /// #4 坚果墙。
    WallNut,
    /// #5 土豆雷。
    PotatoMine,
    /// #6 寒冰射手。
    SnowPea,
    /// #7 大嘴花。
    Chomper,
    /// #8 双发射手。
    Repeater,
    /// #9 小喷菇。
    PuffShroom,
    /// #10 阳光菇。
    SunShroom,
}

impl PlantType {
    /// 代码中登记的植物种类，用于启动时校验 `plants.ron` 键齐全。
    pub const ALL: [Self; 10] = [
        Self::Peashooter,
        Self::Sunflower,
        Self::CherryBomb,
        Self::WallNut,
        Self::PotatoMine,
        Self::SnowPea,
        Self::Chomper,
        Self::Repeater,
        Self::PuffShroom,
        Self::SunShroom,
    ];

    /// RON 中的字符串键名。
    #[must_use]
    pub fn ron_key(self) -> &'static str {
        match self {
            Self::Peashooter => "Peashooter",
            Self::Sunflower => "Sunflower",
            Self::CherryBomb => "CherryBomb",
            Self::WallNut => "WallNut",
            Self::PotatoMine => "PotatoMine",
            Self::SnowPea => "SnowPea",
            Self::Chomper => "Chomper",
            Self::Repeater => "Repeater",
            Self::PuffShroom => "PuffShroom",
            Self::SunShroom => "SunShroom",
        }
    }
}

/// 启动时从 `assets/data/plants.ron` 读入并校验：须含 [`PlantType::ALL`] 全部键。
#[derive(Resource, Debug)]
pub struct PlantsCatalog {
    entries: HashMap<String, PlantArchetypeStats>,
}

impl PlantsCatalog {
    /// 从项目根相对路径读取并解析；键不全或与预期数量不符时 panic（启动期失败即修复数据）。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取植物配置 {full:?}: {e}"));
        let entries: HashMap<String, PlantArchetypeStats> = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析植物配置 {full:?}: {e}"));
        if entries.len() != PlantType::ALL.len() {
            let keys: Vec<_> = entries.keys().cloned().collect();
            panic!(
                "plants.ron 条目数须与 PlantType::ALL 一致（{}），实际 {} 条: {keys:?}",
                PlantType::ALL.len(),
                entries.len()
            );
        }
        for ty in PlantType::ALL {
            let key = ty.ron_key();
            let stats = entries
                .get(key)
                .unwrap_or_else(|| panic!("plants.ron 缺少 PlantType::ALL 对应条目: {key}"));
            validate_plant_entry(ty, stats);
        }
        Self { entries }
    }

    /// 按枚举取一条植物配置。
    #[must_use]
    pub fn get(&self, ty: PlantType) -> Option<&PlantArchetypeStats> {
        self.entries.get(ty.ron_key())
    }

    /// 已加载的植物条目数（启动校验后与 [`PlantType::ALL`] 长度一致）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
