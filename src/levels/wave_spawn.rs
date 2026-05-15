//! 出怪点数（`score`）与权重（`weight`）、最早波次（`min_wave`）协同的抽取逻辑。

use rand::Rng;

use crate::zombies::{ZombieArchetypeStats, ZombieType, ZombiesCatalog};

/// `wave_index` 从 **1** 起，与关卡 `waves` 数组下标关系：`wave_index = index + 1`。
#[allow(dead_code)]
#[must_use]
pub fn spawn_candidates_for_wave(
    catalog: &ZombiesCatalog,
    wave_index: u32,
) -> Vec<(ZombieType, &ZombieArchetypeStats)> {
    ZombieType::ALL
        .iter()
        .copied()
        .filter_map(|ty| catalog.get(ty).map(|s| (ty, s)))
        .filter(|(_, s)| {
            if !s.participates_in_point_spawn_pool() {
                return false;
            }
            wave_index >= s.min_wave
        })
        .collect()
}

/// 在预算内、按 `weight` 加权随机选一种僵尸；仅考虑 `score <= budget` 且 `weight > 0` 的条目。
#[allow(dead_code)]
#[must_use]
pub fn pick_weighted_spawn_kind<R: Rng + ?Sized>(
    catalog: &ZombiesCatalog,
    wave_index: u32,
    budget: i32,
    rng: &mut R,
) -> Option<ZombieType> {
    let pool: Vec<_> = spawn_candidates_for_wave(catalog, wave_index)
        .into_iter()
        .filter(|(_, s)| s.score > 0 && s.score <= budget && s.weight > 0)
        .collect();
    if pool.is_empty() {
        return None;
    }
    let total: u64 = pool
        .iter()
        .map(|(_, s)| s.weight as u64)
        .sum();
    if total == 0 {
        return None;
    }
    let mut r = rng.gen_range(0..total);
    for (ty, s) in &pool {
        let w = s.weight as u64;
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
    wave_index: u32,
    max_points: i32,
    rng: &mut R,
) -> Vec<ZombieType> {
    let mut budget = max_points;
    let mut out = Vec::new();
    while budget > 0 {
        let Some(ty) = pick_weighted_spawn_kind(catalog, wave_index, budget, rng) else {
            break;
        };
        let cost = catalog
            .get(ty)
            .expect("pick_weighted_spawn_kind 只返回目录中存在的种类")
            .score;
        budget -= cost;
        out.push(ty);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zombies::ZombiesCatalog;

    #[test]
    fn wave_1_excludes_conehead_until_min_wave() {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");
        let w1 = spawn_candidates_for_wave(&catalog, 1);
        assert_eq!(w1.len(), 1);
        assert_eq!(w1[0].0, ZombieType::Normal);
        let w2 = spawn_candidates_for_wave(&catalog, 2);
        assert_eq!(w2.len(), 1);
        let w3 = spawn_candidates_for_wave(&catalog, 3);
        assert_eq!(w3.len(), 2);
    }

    #[test]
    fn flag_zombie_only_in_pool_from_wave_10() {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");
        for wave in 1..=9 {
            let pool = spawn_candidates_for_wave(&catalog, wave);
            assert!(
                !pool.iter().any(|(t, _)| *t == ZombieType::FlagZombie),
                "第 {wave} 波不应含旗帜（min_wave=10）"
            );
        }
        let w10 = spawn_candidates_for_wave(&catalog, 10);
        assert_eq!(w10.len(), 4);
        assert!(w10.iter().any(|(t, _)| *t == ZombieType::FlagZombie));
    }

    #[test]
    fn wave_4_includes_pole_vault() {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");
        let w4 = spawn_candidates_for_wave(&catalog, 4);
        assert_eq!(w4.len(), 3);
        assert!(w4.iter().any(|(t, _)| *t == ZombieType::PoleVaultZombie));
    }

    #[test]
    fn roll_wave_3_eventually_draws_conehead() {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");
        let mut rng = rand::thread_rng();
        let mut saw_cone = false;
        for _ in 0..200 {
            let picks = roll_wave_unit_kinds(&catalog, 3, 2000, &mut rng);
            if picks.iter().any(|t| *t == ZombieType::Conehead) {
                saw_cone = true;
                break;
            }
        }
        assert!(
            saw_cone,
            "第 3 波池含路障且预算充足时，多次随机应能抽到 Conehead"
        );
    }

    #[test]
    fn roll_wave_1_only_normal() {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");
        let mut rng = rand::thread_rng();
        for _ in 0..32 {
            let picks = roll_wave_unit_kinds(&catalog, 1, 500, &mut rng);
            assert!(
                picks.iter().all(|t| *t == ZombieType::Normal),
                "第 1 波 min_wave 过滤后应仅有 Normal"
            );
        }
    }
}
