use bevy::prelude::*;

use crate::card_select::{CardSelectPending, SelectedCards};
use crate::levels::{AdventureProgress, CurrentLevel};
use crate::states::GameState;

/// 选卡状态生命周期（UI 在 `ui::card_select`）。
pub struct CardSelectPlugin;

impl Plugin for CardSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CardSelect), card_select_on_enter)
            .add_systems(OnEnter(GameState::Playing), commit_loadout_on_enter_playing)
            .add_systems(OnExit(GameState::Playing), remove_selected_cards)
            // 从选卡返回主菜单等路径：清理未提交的 pending（勿用 OnExit(CardSelect) 读 NextState，过渡时已被消费）
            .add_systems(
                OnTransition {
                    exited: GameState::CardSelect,
                    entered: GameState::MainMenu,
                },
                cleanup_card_select_pending,
            );
    }
}

fn card_select_on_enter(
    progress: Res<AdventureProgress>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if progress.unlocked_plants.len() <= progress.slot_count as usize {
        commands.insert_resource(CardSelectPending {
            selected: progress.unlocked_plants.clone(),
            confirmed: true,
            cancel: false,
        });
        next_state.set(GameState::Playing);
    } else {
        commands.insert_resource(CardSelectPending::default());
    }
}

/// 选卡中途放弃时清理（`CardSelect` → `MainMenu`；→ `Playing` 走 [`commit_loadout_on_enter_playing`]）。
fn cleanup_card_select_pending(mut commands: Commands) {
    commands.remove_resource::<CardSelectPending>();
    commands.remove_resource::<SelectedCards>();
    commands.remove_resource::<CurrentLevel>();
}

/// 进入对战时写入 [`SelectedCards`]（不在选卡态提前写入，避免跳状态时泄漏）。
pub fn commit_loadout_on_enter_playing(
    mut commands: Commands,
    pending: Option<Res<CardSelectPending>>,
) {
    let Some(pending) = pending else {
        panic!("进入 Playing 时缺少 CardSelectPending");
    };
    assert!(
        !pending.selected.is_empty(),
        "选卡结果不能为空"
    );
    commands.insert_resource(SelectedCards {
        plants: pending.selected.clone(),
    });
    commands.remove_resource::<CardSelectPending>();
}

fn remove_selected_cards(mut commands: Commands) {
    commands.remove_resource::<SelectedCards>();
}
