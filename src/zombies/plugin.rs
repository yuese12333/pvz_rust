use bevy::prelude::*;

use crate::zombies::{format_zombie_hp_display, ZombieType, ZombiesCatalog};

/// 僵尸数据与后续生成 / 移动逻辑入口。
pub struct ZombiesPlugin;

impl Plugin for ZombiesPlugin {
    fn build(&self, app: &mut App) {
        let catalog = ZombiesCatalog::load_from_manifest_relative("assets/data/zombies.ron");

        let normal = catalog
            .get(ZombieType::Normal)
            .expect("Normal 僵尸配置须存在");
        assert_eq!(normal.hp_display_string(), "270");
        let _roll_smoke = normal.roll_secs_per_cell(&mut rand::thread_rng());

        let flag = catalog
            .get(ZombieType::FlagZombie)
            .expect("FlagZombie 配置须存在");
        assert_eq!(flag.hp_display_string(), "270");
        assert_eq!(flag.score, 1);
        assert_eq!(flag.weight, 100);
        assert_eq!(flag.min_wave, 10);
        assert_eq!(flag.roll_secs_per_cell(&mut rand::thread_rng()), 3.7);

        let cone = catalog
            .get(ZombieType::Conehead)
            .expect("Conehead 僵尸配置须存在");
        assert_eq!(cone.hp_display_string(), "370（一类）+270");

        let pole = catalog
            .get(ZombieType::PoleVaultZombie)
            .expect("PoleVaultZombie 配置须存在");
        assert_eq!(pole.hp_display_string(), "720");
        assert!((2.4..=2.6).contains(&pole.roll_secs_per_cell(&mut rand::thread_rng())));
        let pv = pole
            .roll_post_vault_secs_per_cell(&mut rand::thread_rng())
            .expect("撑杆跳后须有 post_vault 区间");
        assert!((4.1..=5.3).contains(&pv));

        assert_eq!(normal.score, 100);
        assert_eq!(normal.weight, 4000);
        assert_eq!(normal.min_wave, 1);
        assert_eq!(cone.score, 150);
        assert_eq!(cone.weight, 2000);
        assert_eq!(cone.min_wave, 3);
        assert_eq!(pole.score, 2);
        assert_eq!(pole.weight, 200);
        assert_eq!(pole.min_wave, 4);

        assert_eq!(
            format_zombie_hp_display(Some(300.0), None, 270.0),
            "300（二类）+270"
        );
        assert_eq!(catalog.len(), 4);

        app.insert_resource(catalog);
    }
}
