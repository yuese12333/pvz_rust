//! 对战种子栏：冷却、阳光校验与选中状态。

use bevy::prelude::*;

use crate::card_select::SelectedCards;
use crate::levels::level_balance::effective_plant_stats;
use crate::levels::CurrentLevel;
use crate::plants::{PlantType, PlantsCatalog};
use crate::sun::SunCount;

/// 单个种子卡槽运行时状态。
#[derive(Debug, Clone)]
pub struct SeedSlot {
    pub plant: PlantType,
    /// 冷却进度：`0` = 刚种下/冷却中，`1` = 可再次种植。
    pub cooldown_progress: f32,
    pub cooldown_secs: f64,
    /// 本关有效阳光费用（`init_playing_shop` 时由 `effective_plant_stats` 写入）。
    pub sun_cost: u32,
    /// 当前阳光是否足够支付 [`Self::sun_cost`]。
    pub affordable: bool,
}

impl SeedSlot {
    #[must_use]
    pub fn new(plant: PlantType, cooldown_secs: f64, sun_cost: u32) -> Self {
        Self {
            plant,
            cooldown_progress: 1.0,
            cooldown_secs,
            sun_cost,
            affordable: true,
        }
    }

    /// 是否可点击选中并种植（冷却完毕且阳光足够）。
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.cooldown_progress >= 1.0 && self.affordable
    }

    /// 种下后重置冷却（进度归零，开始计时）。
    pub fn start_cooldown(&mut self) {
        self.cooldown_progress = 0.0;
    }
}

/// 本局种子栏（由 [`SelectedCards`] 初始化）。
#[derive(Resource, Debug)]
pub struct SeedShop {
    pub slots: Vec<SeedSlot>,
}

/// 当前选中的种子（点击卡槽切换）。
#[derive(Resource, Debug, Default)]
pub struct SelectedSeed {
    pub plant: Option<PlantType>,
}

/// 进入对战：阳光 + 种子栏。
pub fn init_playing_shop(
    level: Res<CurrentLevel>,
    selected: Res<SelectedCards>,
    plants: Res<PlantsCatalog>,
    mut commands: Commands,
) {
    let initial_sun = level.inner.initial_sun;
    commands.insert_resource(SunCount(initial_sun));
    let slots = selected
        .plants
        .iter()
        .map(|ty| {
            let stats = effective_plant_stats(plants.as_ref(), &level.inner, *ty).unwrap_or_else(|| {
                panic!("SelectedCards 含未配置植物: {}", ty.ron_key())
            });
            let mut slot = SeedSlot::new(*ty, stats.cooldown_secs, stats.sun_cost);
            slot.affordable = initial_sun >= stats.sun_cost;
            slot
        })
        .collect();
    commands.insert_resource(SeedShop { slots });
    commands.init_resource::<SelectedSeed>();
}

pub fn cleanup_playing_shop(mut commands: Commands) {
    commands.remove_resource::<SunCount>();
    commands.remove_resource::<SeedShop>();
    commands.remove_resource::<SelectedSeed>();
}

pub fn tick_seed_cooldowns(time: Res<Time>, mut shop: ResMut<SeedShop>) {
    let delta = time.delta_secs();
    for slot in &mut shop.slots {
        if slot.cooldown_progress >= 1.0 {
            continue;
        }
        if slot.cooldown_secs <= 0.0 {
            slot.cooldown_progress = 1.0;
            continue;
        }
        slot.cooldown_progress =
            (slot.cooldown_progress + delta / slot.cooldown_secs as f32).min(1.0);
    }
}

pub fn update_seed_affordability(sun: Res<SunCount>, mut shop: ResMut<SeedShop>) {
    if !sun.is_changed() {
        return;
    }
    for slot in &mut shop.slots {
        slot.affordable = sun.0 >= slot.sun_cost;
    }
}

/// 种植成功后由玩法系统调用，重置对应卡槽冷却。
pub fn start_cooldown_for_plant(shop: &mut SeedShop, plant: PlantType) {
    for slot in &mut shop.slots {
        if slot.plant == plant {
            slot.start_cooldown();
            break;
        }
    }
}
