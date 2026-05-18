//! 按关卡 id 加载 `assets/data/levels/{id}.ron` 并校验平衡配置。

use crate::game_data;
use crate::levels::data::LevelDef;
use crate::levels::level_balance::validate_level_balance_config;
use crate::armors::ArmorsCatalog;
use crate::plants::PlantsCatalog;
use crate::zombies::ZombiesCatalog;

/// 关卡清单加载失败原因。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadLevelError {
    /// RON 反序列化失败。
    Ron {
        path: String,
        message: String,
    },
    /// `background` 为空。
    EmptyBackground {
        level_id: String,
    },
}

impl std::fmt::Display for LoadLevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ron { path, message } => write!(f, "加载 {path} 失败: {message}"),
            Self::EmptyBackground { level_id } => {
                write!(f, "关卡 {level_id} 的 background 不能为空")
            }
        }
    }
}

/// `assets/data/levels/{level_id}.ron` 相对项目根的路径。
#[must_use]
pub fn level_manifest_path(level_id: &str) -> String {
    format!("assets/data/levels/{level_id}.ron")
}

/// 加载关卡 RON（不校验僵尸/植物池）。
pub fn load_level_manifest(level_id: &str) -> Result<LevelDef, LoadLevelError> {
    let path = level_manifest_path(level_id);
    let def: LevelDef = game_data::load_ron(&path).map_err(|e| LoadLevelError::Ron {
        path: path.clone(),
        message: e.to_string(),
    })?;
    if def.background.is_empty() {
        return Err(LoadLevelError::EmptyBackground {
            level_id: level_id.to_string(),
        });
    }
    Ok(def)
}

/// 加载关卡并校验与全局僵尸/植物目录及关卡覆盖字段一致。
pub fn load_level_validated(
    level_id: &str,
    zombies: &ZombiesCatalog,
    plants: &PlantsCatalog,
    armors: &ArmorsCatalog,
) -> Result<LevelDef, LoadLevelError> {
    let def = load_level_manifest(level_id)?;
    validate_level_balance_config(&def, zombies, plants, armors);
    Ok(def)
}

/// 启动冒烟：加载失败时 panic（与植物/僵尸目录加载一致）。
pub fn load_level_validated_or_panic(
    level_id: &str,
    zombies: &ZombiesCatalog,
    plants: &PlantsCatalog,
    armors: &ArmorsCatalog,
) -> LevelDef {
    load_level_validated(level_id, zombies, plants, armors).unwrap_or_else(|e| {
        panic!("关卡 {level_id} 加载/校验失败: {e}");
    })
}
