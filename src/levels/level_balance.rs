//! 关卡对 `zombies.ron` / `plants.ron` / `armor.ron` 的局部覆盖、合并与校验。

use crate::armors::{validate_armor_archetype, ArmorArchetypeStats, ArmorType, ArmorsCatalog};
use crate::levels::data::LevelDef;
use crate::plants::{validate_plant_archetype, PlantArchetypeStats, PlantType, PlantsCatalog};
use crate::zombies::{validate_zombie_archetype, ZombieArchetypeStats, ZombieType, ZombiesCatalog};

/// 合并 `zombies.ron` 与关卡覆盖后的完整僵尸数值（本关有效）。
#[must_use]
pub fn effective_zombie_stats(
    catalog: &ZombiesCatalog,
    level: &LevelDef,
    ty: ZombieType,
) -> Option<ZombieArchetypeStats> {
    let base = catalog.get(ty)?;
    let merged = match level.zombie_overrides.as_ref().and_then(|m| m.get(ty.ron_key())) {
        Some(ov) => ov.apply_to(base),
        None => base.clone(),
    };
    Some(merged)
}

/// 合并 `armor.ron` 与关卡覆盖后的防具配置（本关有效）。
#[allow(dead_code)]
#[must_use]
pub fn effective_armor_stats(
    catalog: &ArmorsCatalog,
    level: &LevelDef,
    ty: ArmorType,
) -> Option<ArmorArchetypeStats> {
    let base = catalog.get(ty)?;
    let merged = match level.armor_overrides.as_ref().and_then(|m| m.get(ty.ron_key())) {
        Some(ov) => ov.apply_to(base),
        None => base.clone(),
    };
    Some(merged)
}

/// 合并 `plants.ron` 与关卡覆盖后的植物配置（本关有效）。
#[must_use]
pub fn effective_plant_stats(
    catalog: &PlantsCatalog,
    level: &LevelDef,
    ty: PlantType,
) -> Option<PlantArchetypeStats> {
    let base = catalog.get(ty)?;
    let merged = match level.plant_overrides.as_ref().and_then(|m| m.get(ty.ron_key())) {
        Some(ov) => ov.apply_to(base),
        None => base.clone(),
    };
    Some(merged)
}

/// 解析某僵尸在本关的有效出怪参数；不在 `zombie_pool` 时返回 `None`。
#[must_use]
pub fn effective_spawn_params(
    catalog: &ZombiesCatalog,
    level: &LevelDef,
    ty: ZombieType,
) -> Option<EffectiveSpawnParams> {
    if !level.allows_spawn_kind(ty) {
        return None;
    }
    let stats = effective_zombie_stats(catalog, level, ty)?;
    let params = EffectiveSpawnParams {
        score: stats.score,
        weight: stats.weight,
        min_wave: stats.min_wave,
    };
    if !params.participates_in_point_spawn_pool() {
        return None;
    }
    Some(params)
}

/// 合并 `zombies.ron` 与关卡覆盖后的出怪参数（仅用于点数池逻辑）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveSpawnParams {
    pub score: i32,
    pub weight: u32,
    pub min_wave: u32,
}

impl EffectiveSpawnParams {
    /// 是否参与「`max_points` + 权重 + `min_wave`」出怪池。
    #[must_use]
    pub fn participates_in_point_spawn_pool(&self) -> bool {
        self.score > 0 && self.weight > 0
    }
}

/// 加载关卡后校验池、覆盖键名及合并后僵尸/植物/防具数值合法。
pub fn validate_level_balance_config(
    level: &LevelDef,
    zombies: &ZombiesCatalog,
    plants: &PlantsCatalog,
    armors: &ArmorsCatalog,
) {
    if let Some(pool) = &level.zombie_pool {
        assert!(
            !pool.is_empty(),
            "zombie_pool 非空时至少须列一种僵尸"
        );
        for key in pool {
            assert_known_zombie_key(key, "zombie_pool");
            assert!(
                zombies.get(zombie_type_from_key(key)).is_some(),
                "zombie_pool 条目 {key} 须在 zombies.ron 中存在"
            );
        }
    }

    if let Some(overrides) = &level.zombie_overrides {
        for (key, ov) in overrides {
            assert_known_zombie_key(key, "zombie_overrides");
            let ty = zombie_type_from_key(key);
            let base = zombies
                .get(ty)
                .unwrap_or_else(|| panic!("zombie_overrides 键 {key} 须在 zombies.ron 中存在"));
            let merged = ov.apply_to(base);
            validate_zombie_archetype(ty, &merged, armors);
        }
    }

    if let Some(overrides) = &level.plant_overrides {
        for (key, ov) in overrides {
            assert_known_plant_key(key, "plant_overrides");
            let ty = plant_type_from_key(key);
            let base = plants
                .get(ty)
                .unwrap_or_else(|| panic!("plant_overrides 键 {key} 须在 plants.ron 中存在"));
            let merged = ov.apply_to(base);
            validate_plant_archetype(ty, &merged);
        }
    }

    if let Some(overrides) = &level.armor_overrides {
        for (key, ov) in overrides {
            assert_known_armor_key(key, "armor_overrides");
            let ty = armor_type_from_key(key);
            let base = armors
                .get(ty)
                .unwrap_or_else(|| panic!("armor_overrides 键 {key} 须在 armor.ron 中存在"));
            let merged = ov.apply_to(base);
            validate_armor_archetype(ty, &merged);
        }
    }
}

fn assert_known_zombie_key(key: &str, context: &str) {
    assert!(
        ZombieType::ALL.iter().any(|ty| ty.ron_key() == key),
        "{context} 含未知僵尸键 {key}（须与 zombies.ron / ZombieType 一致）"
    );
}

fn assert_known_plant_key(key: &str, context: &str) {
    assert!(
        PlantType::ALL.iter().any(|ty| ty.ron_key() == key),
        "{context} 含未知植物键 {key}（须与 plants.ron / PlantType 一致）"
    );
}

fn assert_known_armor_key(key: &str, context: &str) {
    assert!(
        ArmorType::ALL.iter().any(|ty| ty.ron_key() == key),
        "{context} 含未知防具键 {key}（须与 armor.ron / ArmorType 一致）"
    );
}

fn zombie_type_from_key(key: &str) -> ZombieType {
    ZombieType::ALL
        .iter()
        .copied()
        .find(|ty| ty.ron_key() == key)
        .unwrap_or_else(|| panic!("内部错误：未解析僵尸键 {key}"))
}

fn plant_type_from_key(key: &str) -> PlantType {
    PlantType::ALL
        .iter()
        .copied()
        .find(|ty| ty.ron_key() == key)
        .unwrap_or_else(|| panic!("内部错误：未解析植物键 {key}"))
}

fn armor_type_from_key(key: &str) -> ArmorType {
    ArmorType::ALL
        .iter()
        .copied()
        .find(|ty| ty.ron_key() == key)
        .unwrap_or_else(|| panic!("内部错误：未解析防具键 {key}"))
}
