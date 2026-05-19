//! 草坪与格线渲染（`Playing` 状态下生成，退出时清理）。

use bevy::prelude::*;

use crate::levels::CurrentLevel;

use super::config::GridConfig;
use super::coords::cell_center;

/// 标记本局网格可视化实体，便于 `OnExit(Playing)` 批量 despawn。
#[derive(Component, Debug, Clone, Copy)]
pub struct GridVisual;

#[derive(Component)]
struct LevelBackgroundVisual;

#[derive(Component)]
struct LawnCellVisual;

#[derive(Component)]
struct GridLineVisual;

const Z_BACKGROUND: f32 = -10.0;
const Z_LAWN: f32 = 0.0;
const Z_GRID_LINES: f32 = 1.0;

/// 进入对战时生成草坪、格线与关卡背景（若资源已加载）。
pub fn spawn_grid_visuals(
    mut commands: Commands,
    config: Res<GridConfig>,
    level: Res<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    let lawn_size = Vec2::new(config.lawn_width(), config.lawn_height());
    let center = config.lawn_center();

    let (br, bg, bb) = config.background_fallback_color;
    commands.spawn((
        GridVisual,
        LevelBackgroundVisual,
        Sprite::from_color(Color::srgb(br, bg, bb), lawn_size),
        Transform::from_xyz(center.x, center.y, Z_BACKGROUND),
    ));

    let bg_handle: Handle<Image> = asset_server.load(level.inner.background.clone());
    commands.spawn((
        GridVisual,
        LevelBackgroundVisual,
        Sprite::from_image(bg_handle),
        Transform::from_xyz(center.x, center.y, Z_BACKGROUND + 0.1),
    ));

    let cell_size = Vec2::new(config.cell_width, config.cell_height);
    let (mr, mg, mb) = config.lawnmower_column_color;
    let mower_color = Color::srgb(mr, mg, mb);
    let (sr, sg, sb) = config.zombie_spawn_column_color;
    let spawn_color = Color::srgb(sr, sg, sb);

    for row in 0..config.rows {
        let row_usize = row as usize;
        let plant_color = {
            let (r, g, b) = config.row_colors[row_usize % config.row_colors.len()];
            Color::srgb(r, g, b)
        };

        for col in 0..config.cols {
            let color = if col == GridConfig::COL_LAWNMOWER {
                mower_color
            } else if col == GridConfig::COL_ZOMBIE_SPAWN {
                spawn_color
            } else {
                plant_color
            };
            let cell_pos = cell_center(&config, col, row);

            commands.spawn((
                GridVisual,
                LawnCellVisual,
                Sprite::from_color(color, cell_size),
                Transform::from_xyz(cell_pos.x, cell_pos.y, Z_LAWN),
            ));
        }
    }

    let (lr, lg, lb) = config.grid_line_color;
    let line_color = Color::srgb(lr, lg, lb);
    let w = config.grid_line_width;
    let o = config.origin_vec2();

    for col in 0..=config.cols {
        let x = o.x + f32::from(col) * config.cell_width;
        commands.spawn((
            GridVisual,
            GridLineVisual,
            Sprite::from_color(line_color, Vec2::new(w, config.lawn_height())),
            Transform::from_xyz(x, center.y, Z_GRID_LINES),
        ));
    }

    for row in 0..=config.rows {
        let y = o.y + f32::from(row) * config.cell_height;
        commands.spawn((
            GridVisual,
            GridLineVisual,
            Sprite::from_color(line_color, Vec2::new(config.lawn_width(), w)),
            Transform::from_xyz(center.x, y, Z_GRID_LINES),
        ));
    }
}

/// 离开对战时移除网格相关实体。
pub fn despawn_grid_visuals(
    mut commands: Commands,
    query: Query<Entity, With<GridVisual>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
