//! Wall related stuff.
//! 
//!  - Spawn walls once at [`StartupStage::PostStartup`] stage.

use super::components::{BallCollider, BoundingBox, Wall};
use crate::consts::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Wall logic as a Bevy’s plugin.
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_wall_system);
    }
}

fn spawn_wall_system(mut commands: Commands) {
    let mut spawn_vertical_wall = |x| {
        let shape = shapes::Rectangle {
            extents: Vec2 {
                x: WALL_THICKNESS * 2.,
                y: WINDOW_HEIGHT,
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
                    translation: Vec3::new(x, 0., WALL_Z_OFFSET),
                    ..Default::default()
                },
            ))
            .insert(BallCollider)
            .insert(Wall)
            .insert(BoundingBox(Vec2::new(WALL_THICKNESS * 2., WINDOW_HEIGHT)));
    };

    spawn_vertical_wall(-WINDOW_WIDTH / 2.);
    spawn_vertical_wall(WINDOW_WIDTH / 2.);

    let horizontal_wall_shape = shapes::Rectangle {
        extents: Vec2 {
            x: WINDOW_WIDTH,
            y: WALL_THICKNESS * 2.,
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
                translation: Vec3::new(0., WINDOW_HEIGHT / 2., WALL_Z_OFFSET),
                ..Default::default()
            },
        ))
        .insert(BallCollider)
        .insert(Wall)
        .insert(BoundingBox(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS * 2.)));
}
