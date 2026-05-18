//! 舞王僵尸行为（见 `mechanics_and_values.md` 三、机制篇 §3）。

use bevy::prelude::*;

/// 舞王行为阶段（按机制篇 §3 顺序驱动）。
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DancingKingPhase {
    /// 滑步入场：1.2 秒/格，约 2.5 格；遇植物提前进入召唤。
    #[default]
    Sliding,
    /// 初次召唤：四行各一只伴舞。
    Summoning,
    /// 舞蹈-移动循环；舞毕检测伴舞数量。
    DancingMoving,
    /// 补员召唤：仅补缺失伴舞。
    RefillSummoning,
}

/// 舞王状态机（移动/召唤系统接入时读写 `phase`）。
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct DancingKing {
    pub phase: DancingKingPhase,
}

/// 滑步入场剩余格数（遭遇植物可提前清零并切至 [`DancingKingPhase::Summoning`]）。
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct DancingSlide {
    pub cells_remaining: f32,
}

/// 伴舞与舞王的从属；舞王死亡后 `leader` 置 `None`，伴舞按普通僵尸独立前进。
#[allow(dead_code)]
#[derive(Component, Debug)]
pub struct BackupDancerFollower {
    pub leader: Option<Entity>,
}

/// 滑步入场目标距离（格），与 RON `secs_per_cell` 1.2 配合。
#[allow(dead_code)]
pub const SLIDE_DISTANCE_CELLS: f32 = 2.5;

/// 召唤/补员时期望伴舞数量（舞王行上下左右各一行）。
#[allow(dead_code)]
pub const BACKUP_DANCER_SLOT_COUNT: u32 = 4;
