//! 网格布局配置（`assets/data/grid.ron`）。
 
use bevy::prelude::*;
use serde::Deserialize;

/// 与 `assets/data/grid.ron` 对齐的草坪网格布局。
#[derive(Resource, Debug, Clone, Deserialize)]
pub struct GridConfig {
    /// 列数（x 方向，僵尸从右向左）；须为 11：
    /// - col 0 — 小推车占位（不可种植）
    /// - col 1..=9 — 可种植区（共 9 列）
    /// - col 10 — 僵尸生成列（屏外缓冲，不参与索敌与种植）
    pub cols: u8,
    /// 行数（y 方向）。
    pub rows: u8,
    /// 单格宽度（世界单位）。
    pub cell_width: f32,
    /// 单格高度（世界单位）；与 `cell_width` 之比为 4:5。
    pub cell_height: f32,
    /// 网格左下角世界坐标 `(x, y)`。
    pub origin: (f32, f32),
    /// 行间交替底色（至少 2 项）；仅用于可种植列 col 1..=9。
    pub row_colors: Vec<(f32, f32, f32)>,
    /// 小推车列（col 0）底色。
    pub lawnmower_column_color: (f32, f32, f32),
    /// 僵尸生成列（col 10）底色。
    pub zombie_spawn_column_color: (f32, f32, f32),
    /// 格线颜色 RGB。
    pub grid_line_color: (f32, f32, f32),
    /// 格线粗细（世界单位）。
    pub grid_line_width: f32,
    /// 关卡背景图未就绪时的整草坪底色。
    pub background_fallback_color: (f32, f32, f32),
}

impl GridConfig {
    /// 小推车所在列。
    #[allow(dead_code)]
    pub const COL_LAWNMOWER: u8 = 0;
    /// 可种植区第一列（最左）。
    pub const COL_PLANT_START: u8 = 1;
    /// 可种植区最后一列（最右）；僵尸进入此列时触发植物索敌。
    pub const COL_PLANT_END: u8 = 9;
    /// 僵尸生成列（屏外缓冲，不参与索敌与种植逻辑）。
    pub const COL_ZOMBIE_SPAWN: u8 = 10;

    /// 从项目根相对路径读取；非法时 panic（启动期 fail fast）。
    #[must_use]
    pub fn load_from_manifest_relative(path: &str) -> Self {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let raw = std::fs::read_to_string(&full)
            .unwrap_or_else(|e| panic!("读取网格配置 {full:?}: {e}"));
        let cfg: Self = ron::de::from_str(&raw)
            .unwrap_or_else(|e| panic!("解析网格配置 {full:?}: {e}"));
        cfg.validate();
        cfg
    }

    fn validate(&self) {
        assert!(
            self.cols == Self::COL_ZOMBIE_SPAWN + 1,
            "grid.cols 须为 {}（小推车列 + 9 可种植列 + 僵尸生成列）",
            Self::COL_ZOMBIE_SPAWN + 1
        );
        assert!(self.rows > 0, "grid.rows 须 > 0");
        assert!(self.cell_width > 0.0, "grid.cell_width 须 > 0");
        assert!(self.cell_height > 0.0, "grid.cell_height 须 > 0");
        let aspect = self.cell_width / self.cell_height;
        assert!(
            (aspect - 0.8).abs() < 0.001,
            "grid 格子宽高比须为 4:5（当前 width/height = {aspect}）"
        );
        assert!(
            self.row_colors.len() >= 2,
            "grid.row_colors 至少 2 项用于行间交替"
        );
        assert!(self.grid_line_width > 0.0, "grid.grid_line_width 须 > 0");
    }

    /// 网格左下角世界坐标。
    #[must_use]
    pub fn origin_vec2(&self) -> Vec2 {
        Vec2::new(self.origin.0, self.origin.1)
    }

    /// 草坪总宽度。
    #[must_use]
    pub fn lawn_width(&self) -> f32 {
        f32::from(self.cols) * self.cell_width
    }

    /// 草坪总高度。
    #[must_use]
    pub fn lawn_height(&self) -> f32 {
        f32::from(self.rows) * self.cell_height
    }

    /// 草坪中心世界坐标（用于背景精灵锚点）。
    #[must_use]
    pub fn lawn_center(&self) -> Vec2 {
        let o = self.origin_vec2();
        Vec2::new(o.x + self.lawn_width() * 0.5, o.y + self.lawn_height() * 0.5)
    }

    /// 列索引是否在草坪内。
    #[must_use]
    #[allow(dead_code)]
    pub fn col_in_bounds(&self, col: u8) -> bool {
        col < self.cols
    }

    /// 行索引是否在草坪内。
    #[must_use]
    #[allow(dead_code)]
    pub fn row_in_bounds(&self, row: u8) -> bool {
        row < self.rows
    }

    /// 该列是否为可种植区。
    #[must_use]
    #[allow(dead_code)]
    pub fn col_is_plantable(&self, col: u8) -> bool {
        col >= Self::COL_PLANT_START && col <= Self::COL_PLANT_END
    }

    /// 该列是否已入场（僵尸进入此列起参与索敌逻辑）。
    #[must_use]
    #[allow(dead_code)]
    pub fn col_is_in_play(col: u8) -> bool {
        col <= Self::COL_PLANT_END
    }
}
