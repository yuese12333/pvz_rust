//! 主菜单 egui 界面与状态跳转（渲染与逻辑分离，便于日后替换布局）。

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game_data;
use crate::levels::CurrentLevel;
use crate::states::GameState;
use crate::ui::AdventureProgress;

/// 主菜单 UI 产生的待处理操作（由逻辑系统消费，渲染层只写入此资源）。
#[derive(Resource, Default)]
struct MainMenuPending {
    start_adventure: bool,
    request_exit: bool,
}

/// 注册主菜单相关系统。
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
        .add_systems(OnExit(GameState::MainMenu), exit_main_menu)
        .add_systems(
            Update,
            (
                draw_main_menu_ui,
                process_main_menu_actions,
            )
                .chain()
                .run_if(in_state(GameState::MainMenu)),
        );
}

fn enter_main_menu(mut commands: Commands) {
    commands.insert_resource(AdventureProgress::load_from_save());
    commands.init_resource::<MainMenuPending>();
}

fn exit_main_menu(mut commands: Commands) {
    commands.remove_resource::<MainMenuPending>();
}

/// 仅负责 egui 绘制；不直接改 [`NextState`] 或加载关卡。
fn draw_main_menu_ui(mut contexts: EguiContexts, mut pending: ResMut<MainMenuPending>) {
    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(48.0);
            ui.heading(
                egui::RichText::new("Plants vs. Zombies")
                    .size(36.0)
                    .strong(),
            );
            ui.add_space(40.0);

            let mode_button_size = egui::vec2(220.0, 40.0);
            if ui
                .add(egui::Button::new("Adventure").min_size(mode_button_size))
                .clicked()
            {
                pending.start_adventure = true;
            }
            ui.add_space(8.0);
            ui.add_enabled(
                false,
                egui::Button::new("Mini-Games").min_size(mode_button_size),
            );
            ui.add_space(8.0);
            ui.add_enabled(
                false,
                egui::Button::new("Puzzle").min_size(mode_button_size),
            );
            ui.add_space(8.0);
            ui.add_enabled(
                false,
                egui::Button::new("Survival").min_size(mode_button_size),
            );

            ui.add_space(48.0);

            ui.horizontal(|ui| {
                let small = egui::vec2(72.0, 28.0);
                ui.add_enabled(false, egui::Button::new("Achievements").min_size(small));
                ui.add_space(6.0);
                ui.add_enabled(false, egui::Button::new("Shop").min_size(small));
                ui.add_space(6.0);
                ui.add_enabled(false, egui::Button::new("Options").min_size(small));
                ui.add_space(6.0);
                if ui.add(egui::Button::new("Quit").min_size(small)).clicked() {
                    pending.request_exit = true;
                }
            });
        });
    });
}

/// 消费 [`MainMenuPending`]：加载关卡、切换状态、退出应用。
fn process_main_menu_actions(
    mut pending: ResMut<MainMenuPending>,
    progress: Res<AdventureProgress>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if pending.request_exit {
        pending.request_exit = false;
        exit.send(AppExit::Success);
        return;
    }

    if !pending.start_adventure {
        return;
    }
    pending.start_adventure = false;

    let path = progress.as_ref().level_manifest_path();
    let level = match game_data::load_ron::<crate::levels::LevelDef>(&path) {
        Ok(def) => def,
        Err(e) => {
            bevy::log::error!("加载关卡 {path} 失败: {e}");
            return;
        }
    };

    info!(
        "开始冒险：加载关卡 {}（initial_sun={}, waves={}）",
        progress.current_level,
        level.initial_sun,
        level.waves.len()
    );
    commands.insert_resource(CurrentLevel { inner: level });
    next_state.set(GameState::Playing);
}
