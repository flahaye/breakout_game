//#![allow(unused)]
// Disable terminal
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use auto_backend::AutoBackendPlugin;
use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_prototype_lyon::prelude::*;
use consts::*;

mod auto_backend;
mod breakout_game;
mod consts;

fn main() {
    App::new()
        .add_plugin(AutoBackendPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
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

fn setup_system(mut commands: Commands, mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(FRAMERATE as f64);
    commands.spawn(Camera2dBundle::default());
}
