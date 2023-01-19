use bevy::{
    prelude::*,
    render::settings::{Backends, WgpuSettings},
};

pub struct AutoBackendPlugin;

impl Plugin for AutoBackendPlugin {
    fn build(&self, app: &mut App) {
        // Setup native Graphics API for each platform.
        let platform_api = if cfg!(target_os = "windows") {
            Backends::DX12
        } else if cfg!(target_os = "macos") {
            Backends::METAL
        } else if cfg!(target_os = "linux") {
            Backends::VULKAN | Backends::GL
        } else {
            Backends::PRIMARY
        };

        app.insert_resource(WgpuSettings {
            backends: Some(platform_api),
            ..Default::default()
        });
    }
}
