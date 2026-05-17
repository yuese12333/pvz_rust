//! 本体三段血量与垂死（见 `mechanics_and_values.md` 三、机制篇 §2.2）。

use bevy::prelude::*;

use crate::game_config::GameConfig;
use crate::zombies::ZombieType;

/// 本体血量阶段（不可逆；当前无回血）。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZombieHpStage {
    /// 正常，比例 ∈ (threshold_high, 1.0]。
    #[default]
    Full,
    /// 掉手，比例 ∈ (threshold_low, threshold_high]。
    #[allow(dead_code)]
    LostArm,
    /// 垂死（掉头后），比例 ∈ [0, threshold_low]。
    Dying,
}

/// 僵尸本体血量与阶段（防具血量另计）。
#[derive(Component, Debug, Clone)]
pub struct ZombieBodyHp {
    #[allow(dead_code)]
    pub ty: ZombieType,
    pub current: f32,
    #[allow(dead_code)]
    pub max: f32,
    pub stage: ZombieHpStage,
}

/// 垂死状态每秒扣血（与植物伤害独立叠加）。
pub fn apply_dying_drain(time: Res<Time>, config: Res<GameConfig>, mut query: Query<&mut ZombieBodyHp>) {
    let delta = config.dying_drain_hp_per_sec * time.delta_seconds();
    if delta <= 0.0 {
        return;
    }
    for mut hp in &mut query {
        if hp.stage != ZombieHpStage::Dying {
            continue;
        }
        hp.current = (hp.current - delta).max(0.0);
    }
}
