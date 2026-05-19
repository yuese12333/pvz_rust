//! 草坪点击种植（`Playing` 状态）。

use bevy::prelude::*;
use bevy::text::{LineBreak, TextBounds};
use bevy_egui::input::EguiWantsInput;

use crate::grid::config::GridConfig;
use crate::grid::coords::{cell_center, world_to_grid, GridPos};
use crate::plants::{plant_sprite_color, Plant, PlantType};
use crate::sun::SunCount;

use super::slots::{start_cooldown_for_plant, SeedShop, SelectedSeed};

const Z_PLANT: f32 = 2.0;
const Z_PLANT_LABEL: f32 = 0.1;
const PLANT_LABEL_FONT_SIZE: f32 = 14.0;
const PLANT_LABEL_PADDING: f32 = 6.0;

/// 鼠标右键取消当前选中的种子。
///
/// 种子栏内由 egui [`egui::Response::secondary_clicked`] 处理；此处仅覆盖草坪等非 UI 区域。
/// `EguiWantsInput` 为真时 egui 已消费指针，不会穿透到本系统。
pub fn deselect_seed_on_right_click(
    egui_wants: Res<EguiWantsInput>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<SelectedSeed>,
) {
    if egui_wants.wants_any_pointer_input() {
        return;
    }
    if mouse.just_pressed(MouseButton::Right) {
        selected.plant = None;
    }
}

/// 鼠标左键在可种植格种下当前选中的种子。
pub fn plant_on_grid_click(
    egui_wants: Res<EguiWantsInput>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    config: Res<GridConfig>,
    mut selected: ResMut<SelectedSeed>,
    mut shop: ResMut<SeedShop>,
    mut sun: ResMut<SunCount>,
    occupied: Query<&GridPos, With<Plant>>,
    mut commands: Commands,
) {
    if egui_wants.wants_any_pointer_input() {
        return;
    }
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let plant_type = match selected.plant {
        Some(p) => p,
        None => return,
    };

    let Ok(window) = window.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(world) = camera.viewport_to_world_2d(cam_transform, cursor) else {
        return;
    };

    let Some(grid_pos) = world_to_grid(&config, world) else {
        return;
    };
    if !config.col_is_plantable(grid_pos.col) {
        return;
    }

    if occupied
        .iter()
        .any(|p| p.col == grid_pos.col && p.row == grid_pos.row)
    {
        return;
    }

    let Some((cost, cooldown_ready)) = shop
        .slots
        .iter()
        .find(|s| s.plant == plant_type)
        .map(|s| (s.sun_cost, s.cooldown_progress >= 1.0))
    else {
        return;
    };
    if !cooldown_ready || sun.0 < cost {
        return;
    }

    sun.0 -= cost;
    start_cooldown_for_plant(&mut shop, plant_type);
    selected.plant = None;

    let center = cell_center(&config, grid_pos.col, grid_pos.row);
    let size = Vec2::new(config.cell_width, config.cell_height);
    spawn_planted_plant(&mut commands, plant_type, grid_pos, center, size);
}

fn spawn_planted_plant(
    commands: &mut Commands,
    plant_type: PlantType,
    grid_pos: GridPos,
    center: Vec2,
    cell_size: Vec2,
) {
    let label = plant_type.ron_key();
    let text_bounds = TextBounds::new(
        cell_size.x - PLANT_LABEL_PADDING * 2.0,
        cell_size.y - PLANT_LABEL_PADDING * 2.0,
    );

    commands
        .spawn((
            Plant,
            plant_type,
            grid_pos,
            Transform::from_xyz(center.x, center.y, Z_PLANT),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_color(plant_sprite_color(plant_type), cell_size),
                Transform::default(),
            ));
            parent.spawn((
                Text2d::new(label),
                TextFont {
                    font_size: PLANT_LABEL_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                TextLayout::new(Justify::Center, LineBreak::WordBoundary),
                text_bounds,
                Transform::from_xyz(0.0, 0.0, Z_PLANT_LABEL),
            ));
        });
}

/// 离开对战时移除已种植实体。
pub fn despawn_planted_plants(mut commands: Commands, query: Query<Entity, With<Plant>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

