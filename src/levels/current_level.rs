//! 当前进行中的关卡数据（进入 [`GameState::Playing`] 时由主菜单写入）。

use bevy::prelude::*;

use super::data::LevelDef;

/// 冒险模式选中的关卡（`Playing` 状态下存在）。
#[derive(Resource, Debug, Clone)]
#[allow(dead_code)]
pub struct CurrentLevel {
    /// 已从 RON 加载的关卡根数据。
    pub inner: LevelDef,
}
