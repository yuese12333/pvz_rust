//! 植物卡片占位色（egui UI 与草坪精灵共用同一套色值）。

use bevy::prelude::Color;
use bevy_egui::egui;

use super::PlantType;

/// 选卡 / 种子栏色块占位色。
#[must_use]
pub fn plant_card_color(ty: PlantType) -> egui::Color32 {
    match ty {
        PlantType::Peashooter => egui::Color32::from_rgb(56, 184, 71),
        PlantType::Sunflower => egui::Color32::from_rgb(242, 209, 46),
        PlantType::CherryBomb => egui::Color32::from_rgb(217, 31, 46),
        PlantType::WallNut => egui::Color32::from_rgb(140, 97, 56),
        PlantType::PotatoMine => egui::Color32::from_rgb(158, 115, 71),
        PlantType::SnowPea => egui::Color32::from_rgb(89, 191, 242),
        PlantType::Chomper => egui::Color32::from_rgb(140, 38, 166),
        PlantType::Repeater => egui::Color32::from_rgb(46, 158, 56),
        PlantType::PuffShroom => egui::Color32::from_rgb(199, 140, 224),
        PlantType::SunShroom => egui::Color32::from_rgb(224, 158, 89),
    }
}

/// 草坪种植精灵占位色（与 [`plant_card_color`] 一致）。
#[must_use]
pub fn plant_sprite_color(ty: PlantType) -> Color {
    to_bevy_color(plant_card_color(ty))
}

fn to_bevy_color(c: egui::Color32) -> Color {
    Color::srgb(
        f32::from(c.r()) / 255.0,
        f32::from(c.g()) / 255.0,
        f32::from(c.b()) / 255.0,
    )
}
