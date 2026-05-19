//! 进关前选卡（与对战内种子栏 `shop` 分离）。

pub mod plugin;

pub use plugin::{CardSelectPlugin, commit_loadout_on_enter_playing};

use bevy::prelude::*;

use crate::plants::PlantType;

/// 本局选卡后带入对战的植物（`CardSelect` → `Playing` 时写入）。
#[derive(Resource, Debug, Clone)]
pub struct SelectedCards {
    pub plants: Vec<PlantType>,
}

/// 选卡 UI 交互状态（仅 `GameState::CardSelect` 期间存在）。
#[derive(Resource, Default)]
pub struct CardSelectPending {
    pub selected: Vec<PlantType>,
    pub confirmed: bool,
    /// 返回主菜单（由 `ui::card_select` 写入）。
    pub cancel: bool,
}
