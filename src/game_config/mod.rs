//! 全局玩法数值（`assets/data/game_config.ron`）。

pub mod plugin;

pub use plugin::GameConfigPlugin;

use bevy::prelude::Resource;
use serde::Deserialize;

/// 与 `assets/data/game_config.ron` 对齐的全局配置。
#[derive(Resource, Debug, Clone, Deserialize)]
pub struct GameConfig {
    /// 本体血量比例上限：剩余比例 **≤** 此值时触发掉手（默认 2/3）。
    pub hp_threshold_high: f32,
    /// 本体血量比例下限：剩余比例 **≤** 此值时进入垂死（默认 1/3）。
    pub hp_threshold_low: f32,
    /// 垂死状态下每秒扣减的本体血量。
    pub dying_drain_hp_per_sec: f32,
}

impl GameConfig {
    /// 从项目根相对路径读取；非法时 panic（启动期 fail fast）。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取全局配置 {full:?}: {e}"));
        let cfg: Self = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析全局配置 {full:?}: {e}"));
        cfg.validate();
        cfg
    }

    fn validate(&self) {
        assert!(
            self.hp_threshold_low > 0.0 && self.hp_threshold_high < 1.0,
            "hp_threshold 须在 (0, 1) 内"
        );
        assert!(
            self.hp_threshold_low < self.hp_threshold_high,
            "hp_threshold_low 须 < hp_threshold_high"
        );
        assert!(
            self.dying_drain_hp_per_sec > 0.0,
            "dying_drain_hp_per_sec 须 > 0"
        );
    }
}
