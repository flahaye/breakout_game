use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    ball::BallPlugin, brick::BrickPlugin, common::CommonPlugin, paddle::PaddlePlugin,
    wall::WallPlugin,
};

pub mod ball;
pub mod brick;
pub mod common;
mod components;
pub mod paddle;
pub mod wall;

pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CorePlugin)
    }
}

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

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::Update, GameStage::Input, SystemStage::parallel())
            .add_stage_before(CoreStage::Update, GameStage::Move, SystemStage::parallel())
            .add_stage_before(
                CoreStage::Update,
                GameStage::Paddle,
                SystemStage::parallel(),
            )
            .add_stage_before(CoreStage::Update, GameStage::Ball, SystemStage::parallel());
    }
}

#[derive(StageLabel)]
pub enum GameStage {
    Input,
    Move,
    Paddle,
    Ball,
}
