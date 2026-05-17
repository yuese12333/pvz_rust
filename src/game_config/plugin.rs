use bevy::prelude::*;

use crate::game_config::GameConfig;

/// 注册 [`GameConfig`]（启动期读取 `assets/data/game_config.ron`）。
pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        let cfg = GameConfig::load_from_manifest_relative("assets/data/game_config.ron");
        assert!(
            (cfg.hp_threshold_high - 0.667).abs() < 1e-3,
            "冒烟：hp_threshold_high 默认约 0.667"
        );
        app.insert_resource(cfg);
    }
}
