//! Paddle related stuff.
//!
//!  - Spawn the paddle once at [`StartupStage::PostStartup`] stage.
//!  - Process input and update [`Velocity`] at [`GameStage::Input`] stage.
//!  - Handle collision of the paddle with walls at [`GameStage::Paddle`] stage.

use super::{
    components::{BallCollider, BoundingBox, Paddle, Velocity, Wall},
    resources::BreakoutConfig,
    GameStage,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_prototype_lyon::prelude::*;

/// Paddle logic as a Bevy’s plugin. (see the game rules)
pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_paddle_system)
            .add_system_to_stage(GameStage::Input, paddle_control_system)
            .add_system_to_stage(GameStage::Paddle, paddle_wall_collision_system);
    }
}

fn spawn_paddle_system(mut commands: Commands, cfg: Res<BreakoutConfig>) {
    let shape = shapes::Rectangle {
        extents: cfg.paddle_size,
        ..Default::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::TURQUOISE),
                outline_mode: StrokeMode::color(Color::BLACK),
            },
            Transform {
                translation: Vec3::new(
                    0.,
                    -cfg.window_height / 2. + cfg.paddle_y_offset,
                    cfg.paddle_z,
                ),
                ..Default::default()
            },
        ))
        .insert(BallCollider)
        .insert(Paddle)
        .insert(Velocity(Vec2::ZERO))
        .insert(BoundingBox(cfg.paddle_size));
}

fn paddle_control_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle>>,
    cfg: Res<BreakoutConfig>,
) {
    let mut x = 0.;

    if keys.pressed(KeyCode::Right) {
        x += cfg.paddle_base_speed;
    } else if keys.pressed(KeyCode::Left) {
        x -= cfg.paddle_base_speed;
    }

    for mut velocity in query.iter_mut() {
        velocity.0.x = x;
    }
}

#[allow(clippy::type_complexity)]
fn paddle_wall_collision_system(
    mut paddle_query: Query<
        (&mut Transform, &BoundingBox, &mut Velocity),
        (With<Paddle>, Without<Wall>),
    >,
    wall_query: Query<(&Transform, &BoundingBox), With<Wall>>,
    cfg: Res<BreakoutConfig>,
) {
    for (mut paddle_tf, paddle_bb, mut paddle_v) in paddle_query.iter_mut() {
        // Clamp the paddle in the window
        paddle_tf.translation.x = paddle_tf.translation.x.clamp(
            -cfg.window_width / 2. + paddle_bb.0.x / 2.,
            cfg.window_width / 2. - paddle_bb.0.x / 2.,
        );

        // Check collision with walls
        for (wall_tf, wall_bb) in wall_query.iter() {
            let collision = collide(
                wall_tf.translation,
                wall_bb.0,
                paddle_tf.translation,
                paddle_bb.0,
            );

            if let Some(Collision::Right) = collision {
                paddle_v.0 = Vec2::ZERO;
                paddle_tf.translation.x =
                    wall_tf.translation.x - wall_bb.0.x / 2. - paddle_bb.0.x / 2.;
            } else if let Some(Collision::Left) = collision {
                paddle_v.0 = Vec2::ZERO;
                paddle_tf.translation.x =
                    wall_tf.translation.x + wall_bb.0.x / 2. + paddle_bb.0.x / 2.;
            }
        }
    }
}
