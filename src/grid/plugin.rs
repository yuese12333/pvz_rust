use bevy::prelude::*;

use crate::states::GameState;

use super::config::GridConfig;
use super::render::{despawn_grid_visuals, spawn_grid_visuals};

/// 网格坐标系与背景渲染。
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        let config = GridConfig::load_from_manifest_relative("assets/data/grid.ron");
        app.insert_resource(config)
            .add_systems(OnEnter(GameState::Playing), spawn_grid_visuals)
            .add_systems(OnExit(GameState::Playing), despawn_grid_visuals);
    }
}
