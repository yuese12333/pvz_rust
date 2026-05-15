//! 从项目根目录读取 `assets/data/*.ron`（仅在 `main` / `FromWorld` / `Plugin::build` 等非 System 路径调用）。

use std::path::PathBuf;

use serde::de::DeserializeOwned;

/// 读取并反序列化相对于 `CARGO_MANIFEST_DIR` 的 RON 文件。
pub fn load_ron<T: DeserializeOwned>(relative_to_manifest: &str) -> Result<T, String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_to_manifest);
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("读取 {path:?}: {e}"))?;
    ron::de::from_str(&raw).map_err(|e| format!("解析 {path:?}: {e}"))
}
