//! 对战内种子栏（冷却、阳光、选中）；不含进关选卡。

pub mod planting;
pub mod plugin;
pub mod slots;
pub mod ui;

pub use plugin::ShopPlugin;
pub use slots::{start_cooldown_for_plant, SeedShop, SelectedSeed};
