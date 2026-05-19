//! 草坪网格坐标系与可视化（见 `.cursor/rules/02-pvz-design.mdc`）。

pub mod config;
pub mod coords;
pub mod plugin;
pub mod render;

#[allow(unused_imports)]
pub use config::GridConfig;
pub use coords::GridPos;
pub use plugin::GridPlugin;
