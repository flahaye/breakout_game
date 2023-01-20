use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallCollider;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub enum BallMoveState {
    FollowPaddle,
    Fly,
}

#[derive(Component)]
pub struct BoundingBox(pub Vec2);

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec2);
