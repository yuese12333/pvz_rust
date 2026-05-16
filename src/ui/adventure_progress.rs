//! 冒险模式进度（当前关卡 id，与 `assets/data/save.ron` 对齐）。

use bevy::prelude::*;
use serde::Deserialize;

/// `assets/data/save.ron` 反序列化结构。
#[derive(Debug, Deserialize)]
struct SaveFile {
    current_level: String,
}

/// 当前冒险关卡文件名（不含 `.ron`），如 `level_1_1`。
#[derive(Resource, Debug, Clone)]
pub struct AdventureProgress {
    /// 对应 `assets/data/levels/{current_level}.ron`。
    pub current_level: String,
}

impl AdventureProgress {
    /// 无存档时的默认关卡。
    pub const DEFAULT_LEVEL: &'static str = "level_1_1";

    /// 从 `assets/data/save.ron` 读取；文件不存在或解析失败时使用 [`Self::DEFAULT_LEVEL`]。
    #[must_use]
    pub fn load_from_save() -> Self {
        match crate::game_data::load_ron::<SaveFile>("assets/data/save.ron") {
            Ok(save) if is_valid_level_id(&save.current_level) => Self {
                current_level: save.current_level,
            },
            Ok(_) => {
                bevy::log::warn!(
                    "save.ron 中 current_level 非法，使用默认 {}",
                    Self::DEFAULT_LEVEL
                );
                Self::default()
            }
            Err(_) => Self::default(),
        }
    }

    /// 关卡 RON 相对于项目根的路径（供 [`crate::game_data::load_ron`] 使用）。
    #[must_use]
    pub fn level_manifest_path(&self) -> String {
        format!("assets/data/levels/{}.ron", self.current_level)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_level_is_level_1_1() {
        let p = AdventureProgress::default();
        assert_eq!(p.current_level, "level_1_1");
    }

    #[test]
    fn rejects_invalid_level_ids() {
        assert!(!is_valid_level_id(""));
        assert!(!is_valid_level_id("../evil"));
        assert!(!is_valid_level_id("a/b"));
    }
}
