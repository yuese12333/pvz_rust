//! 网格坐标与世界坐标换算。

use bevy::prelude::*;

use super::config::GridConfig;

/// 草坪格子坐标（列 `0..cols-1`，行 `0..rows-1`）。
#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub col: u8,
    pub row: u8,
}

#[allow(dead_code)]
impl GridPos {
    /// 若 `(col, row)` 在 [`GridConfig`] 范围内则返回 `Some`。
    #[must_use]
    pub fn new(config: &GridConfig, col: u8, row: u8) -> Option<Self> {
        if config.col_in_bounds(col) && config.row_in_bounds(row) {
            Some(Self { col, row })
        } else {
            None
        }
    }
}

/// 格子中心的世界坐标（`z` 由调用方设置）。
#[must_use]
#[allow(dead_code)]
pub fn cell_center(config: &GridConfig, col: u8, row: u8) -> Vec2 {
    let o = config.origin_vec2();
    Vec2::new(
        o.x + f32::from(col) * config.cell_width + config.cell_width * 0.5,
        o.y + f32::from(row) * config.cell_height + config.cell_height * 0.5,
    )
}

/// 格子左下角世界坐标。
#[must_use]
#[allow(dead_code)]
pub fn cell_bottom_left(config: &GridConfig, col: u8, row: u8) -> Vec2 {
    let o = config.origin_vec2();
    Vec2::new(
        o.x + f32::from(col) * config.cell_width,
        o.y + f32::from(row) * config.cell_height,
    )
}

/// 世界坐标 → 格子坐标；点在草坪外或压线时返回 `None`。
#[must_use]
#[allow(dead_code)]
pub fn world_to_grid(config: &GridConfig, world: Vec2) -> Option<GridPos> {
    let o = config.origin_vec2();
    if world.x < o.x || world.y < o.y {
        return None;
    }
    let rel = world - o;
    let col = (rel.x / config.cell_width).floor() as i32;
    let row = (rel.y / config.cell_height).floor() as i32;
    if col < 0 || row < 0 {
        return None;
    }
    GridPos::new(config, col as u8, row as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> GridConfig {
        GridConfig {
            cols: 11,
            rows: 5,
            cell_width: 80.0,
            cell_height: 100.0,
            origin: (-320.0, -210.0),
            row_colors: vec![(0.28, 0.58, 0.22), (0.24, 0.52, 0.19)],
            lawnmower_column_color: (0.42, 0.44, 0.48),
            zombie_spawn_column_color: (0.16, 0.20, 0.26),
            grid_line_color: (0.14, 0.32, 0.12),
            grid_line_width: 2.0,
            background_fallback_color: (0.2, 0.45, 0.18),
        }
    }

    #[test]
    fn cell_center_matches_design_origin() {
        let cfg = sample_config();
        let c = cell_center(&cfg, 0, 0);
        assert_eq!(c, Vec2::new(-280.0, -160.0));
        let c = cell_center(&cfg, 10, 4);
        assert_eq!(c, Vec2::new(520.0, 240.0));
    }

    #[test]
    fn world_to_grid_round_trip() {
        let cfg = sample_config();
        let pos = world_to_grid(&cfg, cell_center(&cfg, 3, 2)).expect("inside lawn");
        assert_eq!(pos, GridPos { col: 3, row: 2 });
    }

    #[test]
    fn world_to_grid_rejects_outside() {
        let cfg = sample_config();
        assert!(world_to_grid(&cfg, Vec2::new(-400.0, 0.0)).is_none());
        assert!(world_to_grid(&cfg, Vec2::new(0.0, 500.0)).is_none());
    }
}
