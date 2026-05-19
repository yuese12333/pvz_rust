use bevy::prelude::*;

use crate::states::GameState;

use super::planting::{despawn_planted_plants, deselect_seed_on_right_click, plant_on_grid_click};
use super::slots::{
    cleanup_playing_shop, init_playing_shop, tick_seed_cooldowns, update_seed_affordability,
};
use super::ui;

/// 选卡结果、对战种子栏与冷却逻辑。
pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        ui::register(app);
        app.add_systems(
            OnEnter(GameState::Playing),
            init_playing_shop.after(crate::card_select::commit_loadout_on_enter_playing),
        )
            .add_systems(
                OnExit(GameState::Playing),
                (cleanup_playing_shop, despawn_planted_plants),
            )
            .add_systems(
                Update,
                (
                    tick_seed_cooldowns,
                    update_seed_affordability,
                    deselect_seed_on_right_click,
                    plant_on_grid_click,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
