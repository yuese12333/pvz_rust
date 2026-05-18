//! 出怪点数（`score`）与权重（`weight`）、最早波次（`min_wave`）协同的抽取逻辑。

use rand::{Rng, RngExt};

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
    let mut r = rng.random_range(0..total);
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
