//! 冒险模式进度（存档位于用户数据目录，与只读 `assets/data/` 分离）。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plants::PlantType;

/// 玩家存档 RON 结构（`save.ron`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveFile {
    current_level: String,
    #[serde(default = "default_slot_count")]
    slot_count: u32,
    #[serde(default = "default_unlocked_plants_raw")]
    unlocked_plants: Vec<String>,
}

fn default_slot_count() -> u32 {
    AdventureProgress::DEFAULT_SLOT_COUNT
}

fn default_unlocked_plants_raw() -> Vec<String> {
    vec![PlantType::Peashooter.ron_key().to_string()]
}

/// 当前冒险进度（关卡、卡槽数、已解锁植物）。
#[derive(Resource, Debug, Clone)]
pub struct AdventureProgress {
    /// 对应 `assets/data/levels/{current_level}.ron`（只读关卡数据）。
    pub current_level: String,
    /// 选卡与对战种子栏槽位数（初始 6，后续可购买增加）。
    pub slot_count: u32,
    /// 已解锁植物（选卡 UI 与自动跳过逻辑使用）。
    pub unlocked_plants: Vec<PlantType>,
}

impl AdventureProgress {
    pub const DEFAULT_LEVEL: &'static str = "level_1_1";
    pub const DEFAULT_SLOT_COUNT: u32 = 6;

    /// 从用户数据目录下的 `save.ron` 读取；不存在、解析失败或 id 非法时使用默认值。
    #[must_use]
    pub fn load_from_save() -> Self {
        match crate::save::read_save_file::<SaveFile>() {
            Ok(save) if is_valid_level_id(&save.current_level) => Self {
                current_level: save.current_level,
                slot_count: save.slot_count.max(1),
                unlocked_plants: parse_unlocked_plants(&save.unlocked_plants),
            },
            Ok(_) => {
                bevy::log::warn!(
                    "存档 current_level 非法，使用默认 {}",
                    Self::DEFAULT_LEVEL
                );
                Self::default()
            }
            Err(e) => {
                bevy::log::debug!("未加载存档（{e}），使用默认 {}", Self::DEFAULT_LEVEL);
                Self::default()
            }
        }
    }
}

impl Default for AdventureProgress {
    fn default() -> Self {
        Self {
            current_level: Self::DEFAULT_LEVEL.to_string(),
            slot_count: Self::DEFAULT_SLOT_COUNT,
            unlocked_plants: vec![PlantType::Peashooter],
        }
    }
}

fn parse_unlocked_plants(raw: &[String]) -> Vec<PlantType> {
    let plants: Vec<_> = raw.iter().filter_map(|k| PlantType::from_ron_key(k)).collect();
    if plants.is_empty() {
        vec![PlantType::Peashooter]
    } else {
        plants
    }
}

fn is_valid_level_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}
