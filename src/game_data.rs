//! 从项目根目录读取只读 `assets/data/*.ron`（关卡/植物/僵尸等打包资源）。
//!
//! 玩家存档见 [`crate::save`]，勿写入 `assets/data/`。
//!
//! # 路径说明（开发期）
//!
//! 当前使用编译期 [`CARGO_MANIFEST_DIR`] 定位仓库内的 `assets/`，`cargo run` 开发时正确。
//! **发布构建**中该目录不存在，须改为相对可执行文件的路径，或通过 Bevy [`AssetServer`] 加载。

use std::path::PathBuf;

use serde::de::DeserializeOwned;

/// 读取并反序列化相对于 `CARGO_MANIFEST_DIR` 的 RON 文件（仅开发期路径，见模块文档）。
pub fn load_ron<T: DeserializeOwned>(relative_to_manifest: &str) -> Result<T, String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_to_manifest);
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("读取 {path:?}: {e}"))?;
    ron::de::from_str(&raw).map_err(|e| format!("解析 {path:?}: {e}"))
}
