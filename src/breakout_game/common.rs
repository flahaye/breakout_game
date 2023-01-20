//! Common behavior through all entities of the game.
//! 
//!  - Apply [`Velocity`] to [`bevy::prelude::Transform`] at [`GameStage::Move`] stage.

use super::{components::Velocity, GameStage};
use crate::consts::FRAMERATE;
use bevy::prelude::*;

/// All common behavior as a Bevy’s plugin.
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(GameStage::Move, movement_system);
    }
}

fn movement_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, velocity) in query.iter_mut() {
        tf.translation += velocity.0.extend(0.) * (1. / FRAMERATE);
    }
}
