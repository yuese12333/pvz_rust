//! 阳光计数资源。

use bevy::prelude::*;

/// 当前持有阳光数（对战内由关卡 `initial_sun` 初始化）。
#[derive(Resource, Debug, Clone, Copy)]
pub struct SunCount(pub u32);
