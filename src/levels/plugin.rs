use bevy::prelude::*;

use crate::game_data;
use crate::levels::data::{LevelDef, VictoryCondition, WaveTrigger};

/// 已加载的 `level_1.ron`（供后续波次系统读取）。
#[derive(Resource, Debug, Clone)]
pub struct LevelOneDef {
    /// 关卡根数据。
    #[allow(dead_code)]
    pub inner: LevelDef,
}

/// 关卡加载与波次调度。
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        let inner: LevelDef = game_data::load_ron("assets/data/levels/level_1.ron")
            .expect("assets/data/levels/level_1.ron 须存在且为合法 RON");
        assert!(
            !inner.background.is_empty(),
            "level_1 须有 background 路径"
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
        app.insert_resource(LevelOneDef { inner });
    }
}
