//! 启动阶段批量预加载资源并显示 egui 进度条。

use std::path::{Path, PathBuf};

use bevy::asset::UntypedHandle;
use bevy::prelude::*;
use bevy::audio::AudioSource;
use bevy::text::Font;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use walkdir::WalkDir;

use crate::loading::ron_asset::{RonBytesAsset, RonBytesAssetLoader};
use crate::states::GameState;

/// 预加载句柄集合（`OnEnter(Loading)` 写入，`OnExit(Loading)` 移除）。
#[derive(Resource, Debug)]
pub struct LoadingAssets {
    /// 经 [`AssetServer::load`] 得到的未类型化句柄。
    pub handles: Vec<UntypedHandle>,
    /// 上一帧统计的已就绪数量（供 UI 显示）。
    pub loaded: usize,
}

impl LoadingAssets {
    /// 待加载资源总数。
    #[must_use]
    pub fn total(&self) -> usize {
        self.handles.len()
    }

    /// 当前已加载（含依赖）的数量。
    #[must_use]
    pub fn loaded_count(&self, asset_server: &AssetServer) -> usize {
        self.handles
            .iter()
            .filter(|handle| asset_server.is_loaded_with_dependencies(handle.id()))
            .count()
    }
}

/// 资源预加载与加载界面。
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<RonBytesAsset>()
            .init_asset_loader::<RonBytesAssetLoader>()
            .add_systems(OnEnter(GameState::Loading), begin_loading)
            .add_systems(
                Update,
                (
                    update_loading_progress,
                    draw_loading_progress_ui,
                )
                    .chain()
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

fn begin_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets_root = manifest_dir.join("assets");
    let paths = collect_preload_paths(&assets_root);

    let mut handles = Vec::with_capacity(paths.len());

    for rel in paths {
        let handle: UntypedHandle = match rel.rsplit_once('.') {
            Some((_, "png")) => asset_server
                .load::<Image>(format!("{rel}#image/png"))
                .untyped(),
            Some((_, "ogg")) => asset_server.load::<AudioSource>(rel.clone()).untyped(),
            Some((_, "ttf")) => asset_server.load::<Font>(rel.clone()).untyped(),
            Some((_, "ron")) => asset_server.load::<RonBytesAsset>(rel.clone()).untyped(),
            _ => continue,
        };
        handles.push(handle);
    }

    let total = handles.len();
    info!("加载：已排队 {total} 个资源待预加载");

    commands.insert_resource(LoadingAssets {
        handles,
        loaded: 0,
    });
}

fn update_loading_progress(
    mut loading: ResMut<LoadingAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    loading.loaded = loading.loaded_count(&asset_server);
    if loading.total() == 0 || loading.loaded >= loading.total() {
        next_state.set(GameState::MainMenu);
    }
}

fn draw_loading_progress_ui(
    mut contexts: EguiContexts,
    loading: Res<LoadingAssets>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let total = loading.total();
    let loaded = loading.loaded;
    let fraction = if total == 0 {
        1.0
    } else {
        loaded as f32 / total as f32
    };
    let percent = (fraction * 100.0).round();

    let ctx = contexts.ctx_mut();
    egui::Area::new(egui::Id::new("loading_progress"))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let bar_width = (window.width() * 0.45).clamp(280.0, 560.0);
            let bar_height = 22.0;
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Loading...")
                        .size(22.0)
                        .strong(),
                );
                ui.add_space(12.0);
                ui.label(format!("{loaded} / {total}  ({percent:.0}%)"));
                ui.add_space(8.0);
                let (rect, _response) =
                    ui.allocate_exact_size(egui::vec2(bar_width, bar_height), egui::Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, 4.0, egui::Color32::from_gray(48));
                let mut fill = rect;
                fill.set_width(rect.width() * fraction.clamp(0.0, 1.0));
                if fill.width() > 0.0 {
                    painter.rect_filled(fill, 4.0, egui::Color32::from_rgb(72, 160, 88));
                }
                painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_gray(120)));
            });
        });
}

fn cleanup_loading(mut commands: Commands) {
    commands.remove_resource::<LoadingAssets>();
}

fn collect_preload_paths(assets_root: &Path) -> Vec<String> {
    let mut paths = Vec::new();

    if assets_root.is_dir() {
        collect_glob_under(assets_root, "textures", &["png"], &mut paths);
        collect_glob_under(assets_root, "audio", &["ogg"], &mut paths);
        collect_fonts(assets_root, &mut paths);
        collect_data_ron(assets_root, &mut paths);
    }

    paths.sort();
    paths.dedup();
    paths
}

fn collect_glob_under(assets_root: &Path, subdir: &str, extensions: &[&str], out: &mut Vec<String>) {
    let root = assets_root.join(subdir);
    if !root.is_dir() {
        return;
    }
    for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str());
        if ext.is_some_and(|e| extensions.contains(&e)) {
            push_asset_relative(assets_root, path, out);
        }
    }
}

fn collect_fonts(assets_root: &Path, out: &mut Vec<String>) {
    let fonts_dir = assets_root.join("fonts");
    if !fonts_dir.is_dir() {
        return;
    }
    for entry in std::fs::read_dir(&fonts_dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("ttf") {
            push_asset_relative(assets_root, &path, out);
        }
    }
}

fn collect_data_ron(assets_root: &Path, out: &mut Vec<String>) {
    let data_dir = assets_root.join("data");
    if !data_dir.is_dir() {
        return;
    }
    for entry in std::fs::read_dir(&data_dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("ron") {
            push_asset_relative(assets_root, &path, out);
        }
    }
}

fn push_asset_relative(assets_root: &Path, file: &Path, out: &mut Vec<String>) {
    let Ok(rel) = file.strip_prefix(assets_root) else {
        return;
    };
    let rel_str = rel.to_string_lossy().replace('\\', "/");
    out.push(rel_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_includes_data_ron_at_root_only() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
        let paths = collect_preload_paths(&root);
        assert!(paths.iter().any(|p| p == "data/display.ron"));
        assert!(paths.iter().any(|p| p == "data/game_config.ron"));
        assert!(paths.iter().any(|p| p == "data/plants.ron"));
        assert!(!paths.iter().any(|p| p.contains("levels/")));
    }
}
