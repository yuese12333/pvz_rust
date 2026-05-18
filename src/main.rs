use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod game_config;
mod game_data;
mod save;
mod loading;
mod states;

use crate::states::GameState;

mod animations;
mod audio;
mod grid;
mod levels;
mod plants;
mod projectiles;
mod shop;
mod sun;
mod ui;
mod zombies;

use animations::AnimationsPlugin;
use game_config::GameConfigPlugin;
use audio::AudioPlugin;
use loading::LoadingPlugin;
use grid::GridPlugin;
use levels::LevelsPlugin;
use plants::PlantsPlugin;
use projectiles::ProjectilesPlugin;
use shop::ShopPlugin;
use sun::SunPlugin;
use ui::UiPlugin;
use zombies::ZombiesPlugin;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DisplayConfig {
    window_width: f32,
    window_height: f32,
    window_title: String,
    clear_color_rgb: (f32, f32, f32),
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    let display: DisplayConfig = game_data::load_ron("assets/data/display.ron")
        .expect("assets/data/display.ron 须存在且为合法 RON");
    let (cr, cg, cb) = display.clear_color_rgb;

    App::new()
        .insert_resource(ClearColor(Color::srgb(cr, cg, cb)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: display.window_title,
                    resolution: (
                        display.window_width.round() as u32,
                        display.window_height.round() as u32,
                    )
                        .into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(EguiPlugin::default())
        .init_state::<GameState>()
        .add_plugins(LoadingPlugin)
        .add_plugins(GameConfigPlugin)
        .add_plugins((
            GridPlugin,
            PlantsPlugin,
            ZombiesPlugin,
            ProjectilesPlugin,
            SunPlugin,
            ShopPlugin,
            AnimationsPlugin,
            LevelsPlugin,
            UiPlugin,
            AudioPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
