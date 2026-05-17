//! 玩家存档路径（用户可写目录，与只读 `assets/data/` 分离）。

use std::fs;
use std::path::PathBuf;

use serde::de::DeserializeOwned;
// use serde::Serialize;

/// 应用标识，用于拼在用户数据目录下（如 `%LOCALAPPDATA%/pvz_rust`）。
pub const APP_SLUG: &str = "pvz_rust";

/// 存档文件名（位于 [`user_data_dir`] 下）。
pub const SAVE_FILE_NAME: &str = "save.ron";

/// 本游戏存档根目录；无法解析平台用户目录时返回 `None`。
#[must_use]
pub fn user_data_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|p| p.join(APP_SLUG))
}

/// 玩家存档 `save.ron` 的绝对路径。
#[must_use]
pub fn save_file_path() -> Option<PathBuf> {
    user_data_dir().map(|d| d.join(SAVE_FILE_NAME))
}

// /// 确保存档目录存在（写入前调用）。
// pub fn ensure_user_data_dir() -> Result<PathBuf, String> {
//     let dir = user_data_dir().ok_or_else(|| "无法解析用户数据目录".to_string())?;
//     fs::create_dir_all(&dir).map_err(|e| format!("创建存档目录 {dir:?}: {e}"))?;
//     Ok(dir)
// }

/// 从用户存档目录读取并反序列化 RON。
pub fn read_save_file<T: DeserializeOwned>() -> Result<T, String> {
    let path = save_file_path().ok_or_else(|| "无法解析用户数据目录".to_string())?;
    if !path.is_file() {
        return Err(format!("存档不存在: {path:?}"));
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("读取 {path:?}: {e}"))?;
    ron::de::from_str(&raw).map_err(|e| format!("解析 {path:?}: {e}"))
}

// /// 将数据序列化为 RON 并写入用户存档目录下的 `save.ron`。
// pub fn write_save_file<T: Serialize>(value: &T) -> Result<(), String> {
//     let dir = ensure_user_data_dir()?;
//     let path = dir.join(SAVE_FILE_NAME);
//     let content = ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default())
//         .map_err(|e| format!("序列化存档: {e}"))?;
//     fs::write(&path, content).map_err(|e| format!("写入 {path:?}: {e}"))
// }
