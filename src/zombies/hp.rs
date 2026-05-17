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
    LostArm,
    /// 垂死（掉头后），比例 ∈ [0, threshold_low]。
    Dying,
}

/// 僵尸本体血量与阶段（防具血量另计）。
#[derive(Component, Debug, Clone)]
pub struct ZombieBodyHp {
    pub ty: ZombieType,
    pub current: f32,
    pub max: f32,
    pub stage: ZombieHpStage,
}

impl ZombieBodyHp {
    /// 新建满血本体（阶段为 [`ZombieHpStage::Full`]）。
    #[must_use]
    pub fn new(ty: ZombieType, max_body_hp: f32) -> Self {
        Self {
            ty,
            current: max_body_hp,
            max: max_body_hp,
            stage: ZombieHpStage::Full,
        }
    }

    /// 剩余血量占满血比例；`max == 0` 时视为 0。
    #[must_use]
    pub fn hp_ratio(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }

    /// 是否可被植物选为攻击目标（垂死不可）。
    #[must_use]
    pub fn is_targetable_by_plants(&self) -> bool {
        self.stage != ZombieHpStage::Dying
    }

    /// 对本体会造成伤害并更新阶段（仅 `has_segmented_hp` 种类走三段规则）。
    pub fn apply_body_damage(&mut self, damage: f32, config: &GameConfig) {
        if damage <= 0.0 {
            return;
        }
        self.current = (self.current - damage).max(0.0);

        if !self.ty.has_segmented_hp() {
            return;
        }

        let ratio = self.hp_ratio();
        update_stage_after_hp_change(&mut self.stage, ratio, config);
    }
}

/// 按当前比例推进阶段；各临界点仅触发一次（阶段只升不降）。
pub fn update_stage_after_hp_change(
    stage: &mut ZombieHpStage,
    ratio: f32,
    config: &GameConfig,
) {
    if *stage == ZombieHpStage::Dying {
        return;
    }
    if *stage == ZombieHpStage::Full && ratio <= config.hp_threshold_high {
        *stage = ZombieHpStage::LostArm;
    }
    if matches!(*stage, ZombieHpStage::Full | ZombieHpStage::LostArm)
        && ratio <= config.hp_threshold_low
    {
        *stage = ZombieHpStage::Dying;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> GameConfig {
        GameConfig {
            hp_threshold_high: 0.667,
            hp_threshold_low: 0.333,
            dying_drain_hp_per_sec: 90.0,
        }
    }

    #[test]
    fn first_cross_high_triggers_lost_arm_once() {
        let cfg = test_config();
        let mut body = ZombieBodyHp::new(ZombieType::Zombie, 270.0);
        body.apply_body_damage(91.0, &cfg);
        assert_eq!(body.stage, ZombieHpStage::LostArm);
        assert!(body.hp_ratio() > cfg.hp_threshold_low);
    }

    #[test]
    fn cross_low_enters_dying() {
        let cfg = test_config();
        let mut body = ZombieBodyHp::new(ZombieType::Zombie, 270.0);
        body.apply_body_damage(181.0, &cfg);
        assert_eq!(body.stage, ZombieHpStage::Dying);
        assert!(!body.is_targetable_by_plants());
    }

    #[test]
    fn big_hit_skips_to_dying() {
        let cfg = test_config();
        let mut body = ZombieBodyHp::new(ZombieType::Zombie, 270.0);
        body.apply_body_damage(200.0, &cfg);
        assert_eq!(body.stage, ZombieHpStage::Dying);
    }

    #[test]
    fn stage_never_regresses() {
        let cfg = test_config();
        let mut body = ZombieBodyHp::new(ZombieType::Zombie, 270.0);
        body.apply_body_damage(91.0, &cfg);
        body.current = 260.0;
        let ratio = body.hp_ratio();
        update_stage_after_hp_change(&mut body.stage, ratio, &cfg);
        assert_eq!(body.stage, ZombieHpStage::LostArm);
    }
}
