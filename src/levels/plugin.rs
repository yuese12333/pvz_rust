use bevy::prelude::*;

use crate::game_data;
use crate::levels::data::{LevelDef, VictoryCondition, WaveTrigger};
use crate::levels::CurrentLevel;
use crate::states::GameState;

/// 关卡加载与波次调度。
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), cleanup_current_level);

        let inner: LevelDef =
            game_data::load_ron("assets/data/levels/level_1_1.ron").expect(
                "assets/data/levels/level_1_1.ron 须存在且为合法 RON（冒烟校验）",
            );
        assert!(
            !inner.background.is_empty(),
            "level_1_1 须有 background 路径"
        );
        assert_eq!(inner.waves.len(), 3);
        assert_eq!(inner.waves[0].max_points, 500);
        assert!(matches!(
            inner.waves[0].trigger,
            WaveTrigger::Time(t) if (t - 30.0).abs() < f32::EPSILON
        ));
        assert!(matches!(
            inner.victory_condition,
            VictoryCondition::AllWavesCleared
        ));
    }
}

fn cleanup_current_level(mut commands: Commands) {
    commands.remove_resource::<CurrentLevel>();
}
