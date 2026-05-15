use bevy::prelude::*;

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
use audio::AudioPlugin;
use grid::GridPlugin;
use levels::LevelsPlugin;
use plants::PlantsPlugin;
use projectiles::ProjectilesPlugin;
use shop::ShopPlugin;
use sun::SunPlugin;
use ui::UiPlugin;
use zombies::ZombiesPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "PvZ Rust".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<GameState>()
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
        .run();
}
