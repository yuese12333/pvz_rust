use bevy::prelude::*;

use crate::armors::ArmorsCatalog;
use crate::states::GameState;
use crate::zombies::{apply_dying_drain, ZombiesCatalog};

/// 僵尸数据与后续生成 / 移动逻辑入口。
pub struct ZombiesPlugin;

impl Plugin for ZombiesPlugin {
    fn build(&self, app: &mut App) {
        let armors = app
            .world()
            .resource::<ArmorsCatalog>();
        let catalog =
            ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron", armors);

        app.insert_resource(catalog)
            .add_systems(Update, apply_dying_drain.run_if(in_state(GameState::Playing)));
    }
}
