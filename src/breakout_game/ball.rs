use super::{
    components::{Ball, BallCollider, BallMoveState, BoundingBox, Brick, Paddle, Velocity},
    GameStage,
};
use crate::consts::*;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    utils::HashSet,
};
use bevy_prototype_lyon::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_ball_system)
            .add_system_to_stage(GameStage::Ball, follow_paddle_system)
            .add_system_to_stage(GameStage::Ball, launch_ball_system)
            .add_system_to_stage(GameStage::Ball, ball_collision_system)
            .add_system_to_stage(GameStage::Ball, reset_ball_system);
    }
}

fn spawn_ball_system(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        ..Default::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BISQUE),
                outline_mode: StrokeMode::color(Color::BLACK),
            },
            Transform {
                translation: Vec3::new(0., -WINDOW_HEIGHT / 2. + BALL_Y_OFFSET, BALL_Z_OFFSET),
                ..Default::default()
            },
        ))
        .insert(Ball)
        .insert(BoundingBox(Vec2::new(BALL_RADIUS * 2., BALL_RADIUS * 2.)))
        .insert(BallMoveState::FollowPaddle)
        .insert(Velocity(Vec2::ZERO));
}

fn follow_paddle_system(
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    mut ball_query: Query<(&BallMoveState, &mut Transform), With<Ball>>,
) {
    if let Ok(paddle_tf) = paddle_query.get_single() {
        for (move_state, mut ball_tf) in ball_query.iter_mut() {
            if let BallMoveState::FollowPaddle = move_state {
                ball_tf.translation.x = paddle_tf.translation.x;
                ball_tf.translation.y =
                    paddle_tf.translation.y + PADDLE_SIZE.y / 2. + BALL_Y_OFFSET + BALL_RADIUS;
            }
        }
    }
}

fn launch_ball_system(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut BallMoveState, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Velocity, (With<Paddle>, Without<Ball>)>,
) {
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Up) {
        if let Ok(paddle_v) = paddle_query.get_single() {
            for (mut move_state, mut ball_v) in query.iter_mut() {
                if let BallMoveState::FollowPaddle = *move_state {
                    *move_state = BallMoveState::Fly;

                    let velocity = if paddle_v.0.x != 0. {
                        Vec2::new(paddle_v.0.x, paddle_v.0.x.abs())
                    } else {
                        Vec2::new(0., 1.)
                    };
                    ball_v.0 = velocity.normalize_or_zero() * BALL_BASE_SPEED;
                    break;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<
        (&mut Transform, &BoundingBox, &mut Velocity),
        (With<Ball>, Without<BallCollider>),
    >,
    wall_query: Query<
        (
            Entity,
            &Transform,
            &BoundingBox,
            Option<&Velocity>,
            Option<&Brick>,
        ),
        With<BallCollider>,
    >,
) {
    let mut despawned_entities = HashSet::new();
    for (mut ball_tf, ball_bb, mut ball_v) in ball_query.iter_mut() {
        // Clamp the ball in the window, except for the bottom
        ball_tf.translation.x = ball_tf.translation.x.clamp(
            -WINDOW_WIDTH / 2. + ball_bb.0.x / 2.,
            WINDOW_WIDTH / 2. - ball_bb.0.x / 2.,
        );
        let upper_limit = WINDOW_HEIGHT / 2. - ball_bb.0.y / 2.;
        if ball_tf.translation.y > upper_limit {
            ball_tf.translation.y = upper_limit;
        }

        // Check collision with "ball collider"
        for (wall_entity, wall_tf, wall_bb, wall_v, brick) in wall_query.iter() {
            if despawned_entities.contains(&wall_entity) {
                continue;
            }

            let wall_v = wall_v.copied().unwrap_or_default();

            let collision = collide(
                wall_tf.translation,
                wall_bb.0,
                ball_tf.translation,
                ball_bb.0,
            );

            if let Some(collision) = collision {
                match collision {
                    Collision::Right => {
                        ball_v.0.x = -ball_v.0.x;
                        ball_tf.translation.x =
                            wall_tf.translation.x - wall_bb.0.x / 2. - ball_bb.0.x / 2.;
                    }
                    Collision::Left => {
                        ball_v.0.x = -ball_v.0.x;
                        ball_tf.translation.x =
                            wall_tf.translation.x + wall_bb.0.x / 2. + ball_bb.0.x / 2.;
                    }
                    Collision::Top => {
                        ball_v.0.y = -ball_v.0.y;
                        ball_tf.translation.y =
                            wall_tf.translation.y - wall_bb.0.y / 2. - ball_bb.0.y / 2.;
                    }
                    Collision::Bottom => {
                        ball_v.0.y = -ball_v.0.y;
                        ball_tf.translation.y =
                            wall_tf.translation.y + wall_bb.0.y / 2. + ball_bb.0.y / 2.;
                    }
                    _ => (),
                }

                ball_v.0 = (ball_v.0.normalize_or_zero() + wall_v.0.normalize_or_zero())
                    .normalize_or_zero()
                    * BALL_BASE_SPEED;

                if brick.is_some() {
                    if let Some(mut entity_commands) = commands.get_entity(wall_entity) {
                        entity_commands.despawn();
                        despawned_entities.insert(wall_entity);
                    }
                }
            }
        }
    }
}

fn reset_ball_system(mut query: Query<(&Transform, &mut BallMoveState, &mut Velocity), With<Ball>>) {
    for (&tf, mut move_state, mut velocity) in query.iter_mut() {
        if tf.translation.x < -WINDOW_WIDTH / 2. - 200.
            || tf.translation.x > WINDOW_WIDTH / 2. + 200.
            || tf.translation.y < -WINDOW_HEIGHT / 2. - 200.
            || tf.translation.y > WINDOW_HEIGHT / 2. + 200.
        {
            velocity.0 = Vec2::ZERO;
            *move_state = BallMoveState::FollowPaddle;
        }
    }
}
