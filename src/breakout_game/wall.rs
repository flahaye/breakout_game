//! Wall related stuff.
//!
//!  - Spawn walls once at [`StartupStage::PostStartup`] stage.

use super::{
    components::{BallCollider, BoundingBox, Wall},
    resources::BreakoutConfig,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Wall logic as a Bevy’s plugin.
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_wall_system);
    }
}

fn spawn_wall_system(mut commands: Commands, cfg: Res<BreakoutConfig>) {
    let mut spawn_vertical_wall = |x| {
        let shape = shapes::Rectangle {
            extents: Vec2 {
                x: cfg.wall_thickness * 2.,
                y: cfg.window_height,
            },
            ..Default::default()
        };

        commands
            .spawn(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::color(Color::GRAY),
                },
                Transform {
                    translation: Vec3::new(x, 0., cfg.wall_z),
                    ..Default::default()
                },
            ))
            .insert(BallCollider)
            .insert(Wall)
            .insert(BoundingBox(Vec2::new(
                cfg.wall_thickness * 2.,
                cfg.window_height,
            )));
    };

    spawn_vertical_wall(-cfg.window_width / 2.);
    spawn_vertical_wall(cfg.window_width / 2.);

    let horizontal_wall_shape = shapes::Rectangle {
        extents: Vec2 {
            x: cfg.window_width,
            y: cfg.wall_thickness * 2.,
        },
        ..Default::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &horizontal_wall_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GRAY),
                outline_mode: StrokeMode::color(Color::GRAY),
            },
            Transform {
                translation: Vec3::new(0., cfg.window_height / 2., cfg.wall_z),
                ..Default::default()
            },
        ))
        .insert(BallCollider)
        .insert(Wall)
        .insert(BoundingBox(Vec2::new(
            cfg.window_width,
            cfg.wall_thickness * 2.,
        )));
}
