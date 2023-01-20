//! This module contains the game as plugins.

use self::{
    ball::BallPlugin, brick::BrickPlugin, common::CommonPlugin, paddle::PaddlePlugin,
    wall::WallPlugin,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod ball;
pub mod brick;
pub mod common;
pub mod components;
pub mod paddle;
pub mod resources;
pub mod wall;

/// Minimal plugins for the game
/// Only add the requirements for the other plugins to works.
pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CorePlugin)
    }
}

/// All the game plugins.
///
/// Need the following to be added in Bevy’s app to works
///  - [`bevy::prelude::DefaultPlugins`]
///  - [`bevy::prelude::Camera2dBundle`]
///  - [`bevy_prototype_lyon::plugin::ShapePlugin`]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(BallPlugin)
            .add(BrickPlugin)
            .add(CommonPlugin)
            .add(PaddlePlugin)
            .add(WallPlugin)
    }
}

/// Core plugin of the game, contains the requirements for the other plugins.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::Update, GameStage::Init, SystemStage::parallel())
            .add_stage_before(CoreStage::Update, GameStage::Input, SystemStage::parallel())
            .add_stage_before(CoreStage::Update, GameStage::Move, SystemStage::parallel())
            .add_stage_before(
                CoreStage::Update,
                GameStage::Paddle,
                SystemStage::parallel(),
            )
            .add_stage_before(CoreStage::Update, GameStage::Ball, SystemStage::parallel());
    }
}

/// Stages of the game, executed in the order of declaration and before [`CoreStage::Update`].
#[derive(StageLabel)]
pub enum GameStage {
    /// Initialize things that need to be reinitialized multiple times.
    /// Bricks are spawned and balls are reset at this stage.
    Init,

    /// Process input needed for following stages.
    Input,

    /// [`components::Velocity`] is applied at this stage.
    Move,

    /// Paddle related systems.
    Paddle,

    /// Ball related systems.
    ///
    /// Should be run after Paddle to get an up to date velocity for collision.
    Ball,
}
