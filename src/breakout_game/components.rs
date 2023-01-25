//! All the components used by the game.

use bevy::prelude::{Component, Vec2};

/// A marker component to identify a stationary ball.
#[derive(Component)]
pub struct StationaryBall;

/// A marker component to identify a flying ball.
#[derive(Component)]
pub struct FlyingBall;

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

/// A bounding box for collision.
/// Contains the size of the bounding rectangle,
/// the bounding box is assumed to be centered according to [`bevy::prelude::Transform`].
/// Rotation and scale are discarded.
///
///  - [`super::paddle`] systems process them at [`super::GameStage::Paddle`] stage.
///  - [`super::ball`] systems process them at [`super::GameStage::Ball`] stage.
#[derive(Component)]
pub struct BoundingBox(pub Vec2);

/// Game’s score.
#[derive(Component)]
pub struct Score(pub i32);

/// Identify the text displaying the game’s score and the corresponding section index.
#[derive(Component)]
pub struct ScoreText {
    /// The section index displaying the score.
    pub section: usize,
}

/// Velocity to apply to [`bevy::prelude::Transform`] at [`super::GameStage::Move`] stage.
#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);
