//! `plants.ron` 单条植物结构及启动校验。

use serde::Deserialize;

use crate::plants::targeting::PlantTargeting;
use crate::plants::PlantType;

/// 单种植物在 `plants.ron` 中的条目（各植物字段不同，未用字段省略）。
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct PlantArchetypeStats {
    pub health: f32,
    pub sun_cost: u32,
    pub cooldown_secs: f64,
    #[serde(
        default,
        deserialize_with = "crate::plants::targeting::deserialize_present",
        serialize_with = "crate::plants::targeting::serialize_optional"
    )]
    pub targeting: Option<PlantTargeting>,
    #[serde(default)]
    pub shoot_interval: f64,
    #[serde(default)]
    pub damage: f32,
    #[serde(default)]
    pub damage_per_projectile: f32,
    #[serde(default)]
    pub projectiles_per_volley: u32,
    #[serde(default)]
    pub produce_interval: f64,
    #[serde(default)]
    pub sun_yield: u32,
    #[serde(default)]
    pub sun_yield_small: u32,
    #[serde(default)]
    pub sun_yield_mature: u32,
    #[serde(default)]
    pub mature_after_secs: f64,
    #[serde(default)]
    pub first_produce_after_secs: (f32, f32),
    #[serde(default)]
    pub explosion_damage: f32,
    #[serde(default)]
    pub explosion_radius_cells: f32,
    #[serde(default)]
    pub fuse_duration: f64,
    #[serde(default)]
    pub emerge_after_secs: (f32, f32),
    #[serde(default)]
    pub swallow_instakill: bool,
    #[serde(default)]
    pub digest_duration_secs: f64,
    #[serde(default)]
    pub swallow_fail_damage: f32,
    #[serde(default)]
    pub slow_duration: f64,
    #[serde(default)]
    pub slow_factor: f32,
    #[serde(default)]
    pub cracked1_threshold: f32,
    #[serde(default)]
    pub cracked2_threshold: f32,
    #[serde(default)]
    pub low_profile: bool,
    #[serde(default)]
    pub shoot_timer_repeating: bool,
    pub sprite_dir: String,
    pub idle_frames: u32,
    #[serde(default)]
    pub shoot_frames: u32,
    #[serde(default)]
    pub placeholder_rgb: (f32, f32, f32),
    #[serde(default)]
    pub size: (f32, f32),
    #[serde(default)]
    pub spawn_z: f32,
}

macro_rules! de_override_field {
    ($t:ty, $fn_name:ident) => {
        fn $fn_name<'de, D>(d: D) -> Result<Option<$t>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(Some(<$t>::deserialize(d)?))
        }
    };
}

de_override_field!(f32, de_opt_f32);
de_override_field!(u32, de_opt_u32);
de_override_field!(f64, de_opt_f64);

/// 关卡对单种植物数值的局部覆盖（RON 中只写需要改的字段）。
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlantArchetypeOverride {
    #[serde(default, deserialize_with = "de_opt_f32")]
    pub health: Option<f32>,
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub sun_cost: Option<u32>,
    #[serde(default, deserialize_with = "de_opt_f64")]
    pub cooldown_secs: Option<f64>,
    #[serde(
        default,
        deserialize_with = "crate::plants::targeting::deserialize_present"
    )]
    pub targeting: Option<PlantTargeting>,
    #[serde(default, deserialize_with = "de_opt_f64")]
    pub shoot_interval: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f32")]
    pub damage: Option<f32>,
    #[serde(default, deserialize_with = "de_opt_f32")]
    pub damage_per_projectile: Option<f32>,
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub projectiles_per_volley: Option<u32>,
    #[serde(default, deserialize_with = "de_opt_f64")]
    pub produce_interval: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_u32")]
    pub sun_yield: Option<u32>,
    #[serde(default, deserialize_with = "de_opt_f32")]
    pub explosion_damage: Option<f32>,
    #[serde(default, deserialize_with = "de_opt_f64")]
    pub slow_duration: Option<f64>,
    #[serde(default, deserialize_with = "de_opt_f32")]
    pub slow_factor: Option<f32>,
}

impl PlantArchetypeOverride {
    /// 将本关覆盖应用到全局基底，得到本关有效数值。
    #[must_use]
    pub fn apply_to(&self, base: &PlantArchetypeStats) -> PlantArchetypeStats {
        let mut s = base.clone();
        macro_rules! set {
            ($field:ident) => {
                if let Some(v) = self.$field {
                    s.$field = v;
                }
            };
        }
        set!(health);
        set!(sun_cost);
        set!(cooldown_secs);
        set!(shoot_interval);
        set!(damage);
        set!(damage_per_projectile);
        set!(projectiles_per_volley);
        set!(produce_interval);
        set!(sun_yield);
        set!(explosion_damage);
        set!(slow_duration);
        set!(slow_factor);
        if let Some(t) = self.targeting {
            s.targeting = Some(t);
        }
        s
    }
}

/// 校验合并后的植物条目（全局或关卡覆盖后均可调用）。
pub fn validate_plant_archetype(ty: PlantType, stats: &PlantArchetypeStats) {
    validate_plant_entry(ty, stats);
}

fn is_positive_finite(cells: f32) -> bool {
    cells.is_finite() && cells > 0.0
}

fn require_targeting_variant(
    key: &str,
    targeting: Option<PlantTargeting>,
    ok: impl FnOnce(PlantTargeting) -> bool,
    label: &str,
) {
    match targeting {
        Some(t) if ok(t) => {}
        Some(_) => panic!("{key} 的 targeting 须为 {label}"),
        None => panic!("{key} 须配置 targeting（{label}）"),
    }
}

fn forbid_targeting(key: &str, targeting: Option<PlantTargeting>) {
    if targeting.is_some() {
        panic!("{key} 不应配置 targeting");
    }
}

pub(crate) fn validate_plant_entry(ty: PlantType, stats: &PlantArchetypeStats) {
    let key = ty.ron_key();
    if stats.health <= 0.0 {
        panic!("{key} 的 health 须 > 0");
    }
    if stats.cooldown_secs < 0.0 {
        panic!("{key} 的 cooldown_secs 不能为负");
    }
    if stats.sprite_dir.is_empty() {
        panic!("{key} 的 sprite_dir 不能为空");
    }

    let targeting = stats.targeting;
    match ty {
        PlantType::Peashooter | PlantType::SnowPea | PlantType::Repeater => {
            require_targeting_variant(key, targeting, |t| t == PlantTargeting::LaneForward, "LaneForward");
        }
        PlantType::PuffShroom | PlantType::Chomper => {
            require_targeting_variant(
                key,
                targeting,
                |t| matches!(t, PlantTargeting::ForwardRange(c) if is_positive_finite(c)),
                "ForwardRange(正数)",
            );
        }
        PlantType::PotatoMine => {
            require_targeting_variant(
                key,
                targeting,
                |t| matches!(t, PlantTargeting::RowRadius(c) if is_positive_finite(c)),
                "RowRadius(正数)",
            );
        }
        PlantType::CherryBomb => {
            require_targeting_variant(key, targeting, |t| t == PlantTargeting::Area3x3, "Area3x3");
        }
        PlantType::Sunflower | PlantType::WallNut | PlantType::SunShroom => {
            forbid_targeting(key, targeting);
        }
    }
}
