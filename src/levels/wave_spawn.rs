//! 出怪点数（`score`）与权重（`weight`）、最早波次（`min_wave`）协同的抽取逻辑。

use rand::Rng;

use crate::levels::data::LevelDef;
use crate::levels::level_balance::{effective_spawn_params, EffectiveSpawnParams};
use crate::zombies::{ZombieType, ZombiesCatalog};

/// `wave_index` 从 **1** 起，与关卡 `waves` 数组下标关系：`wave_index = index + 1`。
#[allow(dead_code)]
#[must_use]
pub fn spawn_candidates_for_wave(
    catalog: &ZombiesCatalog,
    level: Option<&LevelDef>,
    wave_index: u32,
) -> Vec<(ZombieType, EffectiveSpawnParams)> {
    ZombieType::ALL
        .iter()
        .copied()
        .filter_map(|ty| {
            let params = match level {
                Some(lvl) => effective_spawn_params(catalog, lvl, ty)?,
                None => {
                    let base = catalog.get(ty)?;
                    if !base.participates_in_point_spawn_pool() {
                        return None;
                    }
                    EffectiveSpawnParams {
                        score: base.score,
                        weight: base.weight,
                        min_wave: base.min_wave,
                    }
                }
            };
            if wave_index < params.min_wave {
                return None;
            }
            Some((ty, params))
        })
        .collect()
}

/// 在预算内、按 `weight` 加权随机选一种僵尸；仅考虑 `score <= budget` 且 `weight > 0` 的条目。
#[allow(dead_code)]
#[must_use]
pub fn pick_weighted_spawn_kind<R: Rng + ?Sized>(
    catalog: &ZombiesCatalog,
    level: Option<&LevelDef>,
    wave_index: u32,
    budget: i32,
    rng: &mut R,
) -> Option<ZombieType> {
    let pool: Vec<_> = spawn_candidates_for_wave(catalog, level, wave_index)
        .into_iter()
        .filter(|(_, p)| p.score > 0 && p.score <= budget && p.weight > 0)
        .collect();
    if pool.is_empty() {
        return None;
    }
    let total: u64 = pool.iter().map(|(_, p)| p.weight as u64).sum();
    if total == 0 {
        return None;
    }
    let mut r = rng.gen_range(0..total);
    for (ty, p) in &pool {
        let w = p.weight as u64;
        if r < w {
            return Some(*ty);
        }
        r -= w;
    }
    pool.last().map(|(ty, _)| *ty)
}

/// 用 `max_points` 预算反复加权抽取，直到无法再买入任一种僵尸（纯逻辑，供波次调度接入）。
#[allow(dead_code)]
#[must_use]
pub fn roll_wave_unit_kinds<R: Rng + ?Sized>(
    catalog: &ZombiesCatalog,
    level: Option<&LevelDef>,
    wave_index: u32,
    max_points: i32,
    rng: &mut R,
) -> Vec<ZombieType> {
    let mut budget = max_points;
    let mut out = Vec::new();
    while budget > 0 {
        let Some(ty) = pick_weighted_spawn_kind(catalog, level, wave_index, budget, rng) else {
            break;
        };
        let cost = match level {
            Some(lvl) => effective_spawn_params(catalog, lvl, ty)
                .expect("pick_weighted_spawn_kind 只返回有效候选")
                .score,
            None => catalog
                .get(ty)
                .expect("pick_weighted_spawn_kind 只返回目录中存在的种类")
                .score,
        };
        budget -= cost;
        out.push(ty);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::levels::data::ZombieArchetypeOverride;
    use crate::levels::data::{LevelDef, VictoryCondition, WaveDef, WaveTrigger};
    use crate::zombies::ZombiesCatalog;
    use std::collections::HashMap;

    fn catalog() -> ZombiesCatalog {
        ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron")
    }

    #[test]
    fn wave_1_excludes_conehead_until_min_wave() {
        let catalog = catalog();
        let w1 = spawn_candidates_for_wave(&catalog, None, 1);
        assert_eq!(w1.len(), 1);
        assert_eq!(w1[0].0, ZombieType::Zombie);
        let w2 = spawn_candidates_for_wave(&catalog, None, 2);
        assert_eq!(w2.len(), 1);
        let w3 = spawn_candidates_for_wave(&catalog, None, 3);
        assert_eq!(w3.len(), 2);
    }

    #[test]
    fn flag_zombie_never_in_point_spawn_pool() {
        let catalog = catalog();
        for wave in 1..=20 {
            let pool = spawn_candidates_for_wave(&catalog, None, wave);
            assert!(
                !pool.iter().any(|(t, _)| *t == ZombieType::FlagZombie),
                "旗帜僵尸 score=0 weight=0，第 {wave} 波不应进入点数池"
            );
        }
    }

    #[test]
    fn wave_4_includes_pole_vault() {
        let catalog = catalog();
        let w4 = spawn_candidates_for_wave(&catalog, None, 4);
        assert_eq!(w4.len(), 3);
        assert!(w4.iter().any(|(t, _)| *t == ZombieType::PoleVaultingZombie));
    }

    #[test]
    fn roll_wave_3_eventually_draws_conehead() {
        let catalog = catalog();
        let mut rng = rand::thread_rng();
        let mut saw_cone = false;
        for _ in 0..200 {
            let picks = roll_wave_unit_kinds(&catalog, None, 3, 2000, &mut rng);
            if picks.iter().any(|t| *t == ZombieType::ConeheadZombie) {
                saw_cone = true;
                break;
            }
        }
        assert!(
            saw_cone,
            "第 3 波池含路障且预算充足时，多次随机应能抽到 ConeheadZombie"
        );
    }

    #[test]
    fn roll_wave_1_only_normal() {
        let catalog = catalog();
        let mut rng = rand::thread_rng();
        for _ in 0..32 {
            let picks = roll_wave_unit_kinds(&catalog, None, 1, 500, &mut rng);
            assert!(
                picks.iter().all(|t| *t == ZombieType::Zombie),
                "第 1 波 min_wave 过滤后应仅有 Zombie"
            );
        }
    }

    #[test]
    fn level_override_brings_conehead_to_wave_1() {
        let catalog = catalog();
        let mut overrides = HashMap::new();
        overrides.insert(
            "ConeheadZombie".to_string(),
            ZombieArchetypeOverride {
                min_wave: Some(1),
                ..ZombieArchetypeOverride::default()
            },
        );
        let level = LevelDef {
            background: String::new(),
            bgm: String::new(),
            initial_sun: 0,
            plant_slots: vec![],
            waves: vec![WaveDef {
                trigger: WaveTrigger::Time(1.0),
                max_points: 500,
                is_final: false,
            }],
            victory_condition: VictoryCondition::AllWavesCleared,
            zombie_pool: None,
            zombie_overrides: Some(overrides),
            plant_overrides: None,
        };
        let w1 = spawn_candidates_for_wave(&catalog, Some(&level), 1);
        assert_eq!(w1.len(), 2);
        assert!(w1.iter().any(|(t, _)| *t == ZombieType::ConeheadZombie));
    }
}
