use bevy::prelude::*;

use crate::armors::ArmorsCatalog;

/// 防具数据目录入口。
pub struct ArmorsPlugin;

impl Plugin for ArmorsPlugin {
    fn build(&self, app: &mut App) {
        let catalog = ArmorsCatalog::load_from_manifest_relative("assets/data/armor.ron");
        app.insert_resource(catalog);
    }
}
