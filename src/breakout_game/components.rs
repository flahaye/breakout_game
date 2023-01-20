//! All the components used by the game.

use bevy::prelude::{Component, Vec2};

/// A marker component to identify balls.
#[derive(Component)]
pub struct Ball;

/// A marker component to identify entities that can collide with balls.
#[derive(Component)]
pub struct BallCollider;

/// A marker component to identify bricks.
#[derive(Component)]
pub struct Brick;

/// A marker component to identify paddles.
#[derive(Component)]
pub struct Paddle;

/// A marker component to identify walls.
#[derive(Component)]
pub struct Wall;

/// State of the ball, following the paddle or flying.
///
/// Processed by [`super::ball`] systems at [`super::GameStage::Ball`] stage.
#[derive(Component)]
pub enum BallMoveState {
    /// The ball follow the paddle, no yet thrown.
    FollowPaddle,

    /// The ball was thrown
    Fly,
}

/// A bounding box for collision.
/// Contains the size of the bounding rectangle,
/// the bounding box is assumed to be centered according to [`bevy::prelude::Transform`].
/// Rotation and scale are discarded.
///
///  - [`super::paddle`] systems process them at [`super::GameStage::Paddle`] stage.
///  - [`super::ball`] systems process them at [`super::GameStage::Ball`] stage.
#[derive(Component)]
pub struct BoundingBox(pub Vec2);

/// Velocity to apply to [`bevy::prelude::Transform`] at [`super::GameStage::Move`] stage.
#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);
