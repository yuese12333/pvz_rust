//! 对战种子栏 egui（`Playing` 状态）。

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};

use crate::plants::plant_card_color;
use crate::shop::{SeedShop, SelectedSeed};
use crate::states::GameState;
use crate::sun::SunCount;

const SLOT_THUMB_SIZE: egui::Vec2 = egui::Vec2::new(40.0, 56.0);

/// 注册种子栏 UI。
pub fn register(app: &mut App) {
    app.add_systems(
        EguiPrimaryContextPass,
        draw_seed_shop_bar.run_if(in_state(GameState::Playing)),
    );
}

fn draw_seed_shop_bar(
    mut contexts: EguiContexts,
    shop: Res<SeedShop>,
    mut selected: ResMut<SelectedSeed>,
    sun: Res<SunCount>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    egui::TopBottomPanel::top("seed_shop_bar")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("☀ {sun}", sun = sun.0));
                ui.separator();
                for slot in &shop.slots {
                    let cost = slot.sun_cost;
                    let ready = slot.is_ready();
                    let is_selected = selected.plant == Some(slot.plant);

                    let name = slot.plant.ron_key();
                    let response = draw_seed_slot_card(
                        ui,
                        slot.plant,
                        name,
                        cost,
                        ready,
                        is_selected,
                        slot.cooldown_progress,
                    );
                    if ready && response.clicked() {
                        selected.plant = Some(slot.plant);
                    }
                    // 种子栏内右键取消；草坪区域由 planting::deselect_seed_on_right_click 处理
                    if response.secondary_clicked() && selected.plant.is_some() {
                        selected.plant = None;
                    }
                    ui.add_space(4.0);
                }
            });
        });
}

fn draw_seed_slot_card(
    ui: &mut egui::Ui,
    plant: crate::plants::PlantType,
    name: &str,
    cost: u32,
    ready: bool,
    is_selected: bool,
    cooldown_progress: f32,
) -> egui::Response {
    let stroke = if is_selected {
        egui::Stroke::new(2.5, egui::Color32::from_rgb(72, 120, 200))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_gray(90))
    };
    let mut frame = egui::Frame::new()
        .stroke(stroke)
        .inner_margin(4.0)
        .corner_radius(4.0);
    if !ready {
        frame = frame.fill(egui::Color32::from_gray(42));
    }

    let inner = frame.show(ui, |ui| {
        ui.set_width(SLOT_THUMB_SIZE.x);
        ui.vertical_centered(|ui| {
            let (thumb_rect, _) = ui.allocate_exact_size(SLOT_THUMB_SIZE, egui::Sense::hover());
            let thumb_color = if ready {
                plant_card_color(plant)
            } else {
                egui::Color32::from_gray(72)
            };
            ui.painter().rect_filled(thumb_rect, 2.0, thumb_color);
            let name_color = if ready {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_gray(180)
            };
            ui.painter().text(
                thumb_rect.center(),
                egui::Align2::CENTER_CENTER,
                name,
                egui::FontId::proportional(8.5),
                name_color,
            );

            if cooldown_progress < 1.0 {
                let frac = 1.0 - cooldown_progress;
                let cover_h = thumb_rect.height() * frac;
                let cover_top = thumb_rect.max.y - cover_h;
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(thumb_rect.min.x, cover_top),
                        egui::vec2(thumb_rect.width(), cover_h),
                    ),
                    0.0,
                    egui::Color32::from_black_alpha(140),
                );
            }

            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(format!("☀{cost}"))
                    .size(11.0)
                    .color(if ready {
                        egui::Color32::from_rgb(220, 180, 40)
                    } else {
                        egui::Color32::from_gray(110)
                    }),
            );
        });
    });
    if ready {
        inner.response.interact(egui::Sense::click())
    } else {
        inner.response
    }
}
