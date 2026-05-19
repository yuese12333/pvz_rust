//! `Playing` 状态占位 UI 与进入关卡日志（正式 HUD / 网格接入前）。

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};

use crate::levels::CurrentLevel;
use crate::states::GameState;
use crate::levels::AdventureProgress;

/// 注册 `Playing` 相关系统。
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), log_enter_playing)
        .add_systems(
            EguiPrimaryContextPass,
            draw_playing_placeholder_ui.run_if(in_state(GameState::Playing)),
        );
}

fn log_enter_playing(level: Res<CurrentLevel>, progress: Res<AdventureProgress>) {
    info!(
        "进入关卡: {}（initial_sun={}, waves={}）",
        progress.current_level,
        level.inner.initial_sun,
        level.inner.waves.len()
    );
}

/// 占位 HUD：显示关卡 id 与返回主菜单（后续由贴图 / 正式 UI 替换）。
fn draw_playing_placeholder_ui(
    mut contexts: EguiContexts,
    _level: Res<CurrentLevel>,
    progress: Res<AdventureProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::TopBottomPanel::bottom("playing_placeholder_hud").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Level: {}", progress.current_level));
            ui.separator();
            ui.label("(Sun shown in seed bar)");
            ui.separator();
            ui.label("(Gameplay WIP)");
            if ui.button("Back to Menu").clicked() {
                next_state.set(GameState::MainMenu);
            }
        });
    });
}
