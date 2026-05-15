use bevy::prelude::*;

use crate::plants::{PlantType, PlantsCatalog};

/// 注册 [`PlantsCatalog`]（启动期读取 `assets/data/plants.ron`）。
pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        let catalog = PlantsCatalog::load_from_manifest_relative("assets/data/plants.ron");
        assert_eq!(
            catalog.len(),
            PlantType::ALL.len(),
            "plants.ron 条目数应与 PlantType::ALL 一致"
        );
        assert!(
            catalog.get(PlantType::Peashooter).is_some(),
            "Peashooter 条目须可查询"
        );
        app.insert_resource(catalog);
    }
}
