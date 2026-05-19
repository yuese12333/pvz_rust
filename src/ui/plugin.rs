use bevy::prelude::*;

use crate::ui::{card_select, main_menu, playing};

/// HUD、主菜单、暂停与胜负界面。
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        main_menu::register(app);
        card_select::register(app);
        playing::register(app);
    }
}
