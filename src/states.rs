//! 游戏状态与 System 执行顺序集合。

use bevy::prelude::*;

/// 顶层游戏状态机。
#[allow(dead_code)]
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    CardSelect,
    Playing,
    Paused,
    Victory,
    Defeat,
}

/// Update 阶段 System 执行顺序。
#[allow(dead_code)]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Logic,
    Spawn,
    Render,
}
