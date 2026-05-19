//! 进关前选卡 egui（逻辑与资源在 `card_select` 模块）。

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};

use crate::card_select::CardSelectPending;
use crate::levels::AdventureProgress;
use crate::plants::{plant_card_color, PlantType, PlantsCatalog};
use crate::states::GameState;

const CARD_THUMB_SIZE: egui::Vec2 = egui::Vec2::new(50.0, 70.0);

/// 注册选卡 UI 系统。
pub fn register(app: &mut App) {
    app.add_systems(
        EguiPrimaryContextPass,
        (
            draw_card_select_ui,
            process_card_select_actions,
        )
            .chain()
            .run_if(in_state(GameState::CardSelect)),
    );
}

fn draw_card_select_ui(
    mut contexts: EguiContexts,
    progress: Res<AdventureProgress>,
    plants: Res<PlantsCatalog>,
    mut pending: ResMut<CardSelectPending>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let max_slots = progress.slot_count as usize;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("选择本关植物");
            ui.label(format!(
                "已选 {} / {}（从 {} 种解锁植物中选）",
                pending.selected.len(),
                max_slots,
                progress.unlocked_plants.len()
            ));
            ui.add_space(16.0);

            ui.horizontal_wrapped(|ui| {
                for ty in &progress.unlocked_plants {
                    let name = ty.ron_key();
                    let cost = plants.get(*ty).map(|s| s.sun_cost).unwrap_or(0);
                    let picked = pending.selected.contains(ty);
                    let can_add = !picked && pending.selected.len() < max_slots;
                    let enabled = picked || can_add;

                    if draw_select_card(ui, *ty, name, cost, picked, enabled).clicked() && enabled {
                        if picked {
                            if let Some(i) = pending.selected.iter().position(|p| *p == *ty) {
                                pending.selected.remove(i);
                            }
                        } else {
                            pending.selected.push(*ty);
                        }
                    }
                    ui.add_space(8.0);
                }
            });

            ui.add_space(24.0);
            let can_confirm =
                !pending.selected.is_empty() && pending.selected.len() <= max_slots;
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(can_confirm, egui::Button::new("开始战斗"))
                    .clicked()
                {
                    pending.confirmed = true;
                }
                if ui.button("返回主菜单").clicked() {
                    pending.cancel = true;
                }
            });
        });
    });
}

fn draw_select_card(
    ui: &mut egui::Ui,
    plant: PlantType,
    name: &str,
    cost: u32,
    picked: bool,
    enabled: bool,
) -> egui::Response {
    let stroke = if picked {
        egui::Stroke::new(2.5, egui::Color32::from_rgb(72, 120, 200))
    } else {
        egui::Stroke::new(1.0, egui::Color32::from_gray(90))
    };
    let mut frame = egui::Frame::new()
        .stroke(stroke)
        .inner_margin(6.0)
        .corner_radius(4.0);
    if !enabled {
        frame = frame.fill(egui::Color32::from_gray(42));
    }

    let inner = frame.show(ui, |ui| {
        ui.set_width(CARD_THUMB_SIZE.x);
        ui.vertical_centered(|ui| {
            let (thumb_rect, _) = ui.allocate_exact_size(CARD_THUMB_SIZE, egui::Sense::hover());
            let thumb_color = if enabled {
                plant_card_color(plant)
            } else {
                egui::Color32::from_gray(72)
            };
            ui.painter().rect_filled(thumb_rect, 2.0, thumb_color);
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(name)
                    .size(12.0)
                    .color(if enabled {
                        ui.visuals().text_color()
                    } else {
                        egui::Color32::from_gray(130)
                    }),
            );
            ui.label(
                egui::RichText::new(format!("☀{cost}"))
                    .size(11.0)
                    .color(if enabled {
                        egui::Color32::from_rgb(220, 180, 40)
                    } else {
                        egui::Color32::from_gray(110)
                    }),
            );
        });
    });
    if enabled {
        inner.response.interact(egui::Sense::click())
    } else {
        inner.response
    }
}

fn process_card_select_actions(
    mut pending: ResMut<CardSelectPending>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if pending.cancel {
        pending.cancel = false;
        next_state.set(GameState::MainMenu);
        return;
    }
    if !pending.confirmed {
        return;
    }
    pending.confirmed = false;
    next_state.set(GameState::Playing);
}
