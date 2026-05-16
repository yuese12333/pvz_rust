//! 通过 [`AssetServer`] 加载的 RON 字节资源（供预加载 `assets/data/*.ron`）。

use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

/// 预加载用 RON 文件内容（原始字节，不在此阶段解析为游戏结构体）。
#[derive(Asset, TypePath, Debug, Clone)]
#[allow(dead_code)]
pub struct RonBytesAsset(pub Vec<u8>);

/// [`RonBytesAsset`] 的加载器。
#[derive(Default)]
pub struct RonBytesAssetLoader;

#[derive(Default, Serialize, Deserialize)]
pub struct RonBytesAssetLoaderSettings;

impl AssetLoader for RonBytesAssetLoader {
    type Asset = RonBytesAsset;
    type Settings = RonBytesAssetLoaderSettings;
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut Reader<'_>,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(RonBytesAsset(bytes))
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
