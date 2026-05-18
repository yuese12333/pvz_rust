use bevy::prelude::*;

use crate::armors::ArmorsCatalog;
use crate::levels::load::load_level_validated_or_panic;
use crate::levels::progress::AdventureProgress;
use crate::levels::CurrentLevel;
use crate::plants::PlantsCatalog;
use crate::states::GameState;
use crate::zombies::ZombiesCatalog;

/// 关卡进度、加载与 `Playing` 生命周期。
pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        let progress = AdventureProgress::load_from_save();
        let armors = app.world().resource::<ArmorsCatalog>();
        let zombies = app.world().resource::<ZombiesCatalog>();
        let plants = app.world().resource::<PlantsCatalog>();
        load_level_validated_or_panic(&progress.current_level, zombies, plants, armors);

        app.insert_resource(progress)
            .add_systems(OnExit(GameState::Playing), cleanup_current_level);
    }
}

fn cleanup_current_level(mut commands: Commands) {
    commands.remove_resource::<CurrentLevel>();
}
