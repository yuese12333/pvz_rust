//! 植物数据层：与 `game_mechanics_and_values.md` 及 `assets/data/plants.ron` 对齐（#1～#10）。
//! 网格 / 商店 / 子弹等逻辑仍在各自 Plugin 中，当前多为占位，由 `main` 统一注册。

pub mod plugin;

pub use plugin::PlantsPlugin;

use std::collections::HashMap;

use bevy::prelude::*;

/// 植物类型（与 `assets/data/plants.ron` 键名一致，顺序与 `game_mechanics_and_values.md` #1～#10 一致）。
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
    /// 文档固定列举的十种植物，用于启动时校验 RON。
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

/// 启动时从 `assets/data/plants.ron` 读入并校验：须含文档 #1～#10 全部键。
#[derive(Resource, Debug)]
pub struct PlantsCatalog {
    /// 各植物条目的 RON 值（结构随植物种类不同，后续玩法再强类型解析）。
    entries: HashMap<String, ron::value::Value>,
}

impl PlantsCatalog {
    /// 从项目根相对路径读取并解析；键不全或与预期数量不符时 panic（启动期失败即修复数据）。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取植物配置 {full:?}: {e}"));
        let entries: HashMap<String, ron::value::Value> = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析植物配置 {full:?}: {e}"));
        for ty in PlantType::ALL {
            let key = ty.ron_key();
            if !entries.contains_key(key) {
                panic!("plants.ron 缺少文档对应条目: {key}");
            }
        }
        if entries.len() != PlantType::ALL.len() {
            let keys: Vec<_> = entries.keys().cloned().collect();
            panic!(
                "plants.ron 仅应包含文档 #1～#10 共 {} 条，实际 {} 条: {keys:?}",
                PlantType::ALL.len(),
                entries.len()
            );
        }
        Self { entries }
    }

    /// 按枚举取一条植物配置（RON 值）。
    #[must_use]
    pub fn get(&self, ty: PlantType) -> Option<&ron::value::Value> {
        self.entries.get(ty.ron_key())
    }

    /// 已加载的植物条目数（启动校验后恒为 10）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
