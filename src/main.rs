//! A simple game of breakout to learn Bevy.

// Disable terminal
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(missing_docs)]
//#![allow(unused)]

use auto_backend::AutoBackendPlugin;
use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_prototype_lyon::prelude::*;
use breakout_game::resources::GameConfig;

mod auto_backend;
pub mod breakout_game;

fn main() {
    let cfg = GameConfig::default();
    let width = cfg.window_width;
    let height = cfg.window_height;
    let background_color = cfg.background_color;

    App::new()
        .add_plugin(AutoBackendPlugin)
        .insert_resource(ClearColor(background_color))
        .insert_resource(cfg)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width,
                height,
                title: "Breakout in Rust/Bevy".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(FramepacePlugin)
        .add_plugin(ShapePlugin)
        .add_plugins(breakout_game::DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut settings: ResMut<FramepaceSettings>,
    cfg: Res<GameConfig>,
) {
    settings.limiter = Limiter::from_framerate(cfg.framerate as f64);
    commands.spawn(Camera2dBundle::default());
}
