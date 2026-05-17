//! 冒险模式进度（存档位于用户数据目录，与只读 `assets/data/` 分离）。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// use super::load::level_manifest_path;

/// 玩家存档 RON 结构（`save.ron`）。
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveFile {
    current_level: String,
}

/// 当前冒险关卡文件名（不含 `.ron`），如 `level_1_1`。
#[derive(Resource, Debug, Clone)]
pub struct AdventureProgress {
    /// 对应 `assets/data/levels/{current_level}.ron`（只读关卡数据）。
    pub current_level: String,
}

impl AdventureProgress {
    /// 无存档时的默认关卡。
    pub const DEFAULT_LEVEL: &'static str = "level_1_1";

    /// 从用户数据目录下的 `save.ron` 读取；不存在、解析失败或 id 非法时使用 [`Self::DEFAULT_LEVEL`]。
    #[must_use]
    pub fn load_from_save() -> Self {
        match crate::save::read_save_file::<SaveFile>() {
            Ok(save) if is_valid_level_id(&save.current_level) => Self {
                current_level: save.current_level,
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

    // /// 将当前进度写入用户数据目录下的 `save.ron`（关卡通关等时机调用）。
    // pub fn save_to_disk(&self) -> Result<(), String> {
    //     if !is_valid_level_id(&self.current_level) {
    //         return Err(format!(
    //             "无法保存非法关卡 id: {:?}",
    //             self.current_level
    //         ));
    //     }
    //     crate::save::write_save_file(&SaveFile {
    //         current_level: self.current_level.clone(),
    //     })
    // }

    // /// 关卡 RON 相对于项目根的路径（供 [`crate::game_data::load_ron`] 使用）。
    // #[must_use]
    // pub fn level_manifest_path(&self) -> String {
    //     level_manifest_path(&self.current_level)
    // }
}

impl Default for AdventureProgress {
    fn default() -> Self {
        Self {
            current_level: Self::DEFAULT_LEVEL.to_string(),
        }
    }
}

fn is_valid_level_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}
