use bevy::prelude::*;

use crate::consts::FRAMERATE;

use super::{components::Velocity, GameStage};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(GameStage::Move, movement_system);
    }
}

pub fn movement_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, velocity) in query.iter_mut() {
        tf.translation += velocity.0.extend(0.) * (1. / FRAMERATE);
    }
}
