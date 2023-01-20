//! Brick related stuff.
//!
//!  - Spawn or respawn bricks at [`GameStage::Init`].
//!  - Insert [`BrickRespawn`] resource.

use super::{
    components::{BallCollider, BoundingBox, Brick},
    resources::BreakoutConfig,
    GameStage,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

/// Brick logic as a Bevy’s plugin. (see the game rules)
pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(GameStage::Init, spawn_brick_system)
            .insert_resource(BrickRespawn {
                immediate_spawn: true,
                timer: Timer::from_seconds(1., TimerMode::Once),
            });
    }
}

/// Indicate if bricks should be spawned or not.
#[derive(Resource)]
pub struct BrickRespawn {
    /// If true, brick will be spawned on the next update, regardless of the timer.
    immediate_spawn: bool,

    /// Bricks will spawn at the end of timer.
    /// Timer is reset and pause after a spawn.
    timer: Timer,
}

fn spawn_brick_system(
    mut commands: Commands,
    time: Res<Time>,
    mut brick_respawn: ResMut<BrickRespawn>,
    bricks_query: Query<&Brick>,
    cfg: Res<BreakoutConfig>,
) {
    if bricks_query.is_empty() {
        brick_respawn.timer.unpause();
    }

    brick_respawn.timer.tick(time.delta());

    if brick_respawn.immediate_spawn || brick_respawn.timer.just_finished() {
        brick_respawn.timer.reset();
        brick_respawn.timer.pause();
        brick_respawn.immediate_spawn = false;

        let mut rng = thread_rng();

        let shape = shapes::Rectangle {
            extents: cfg.brick_size,
            ..Default::default()
        };

        let brick_area_size = Vec2::new(
            cfg.brick_area_cols as f32 * cfg.brick_size.x,
            cfg.brick_area_rows as f32 * cfg.brick_size.y,
        );

        // bottom left brick
        let first_brick_translation = Vec2::new(
            -brick_area_size.x / 2. + cfg.brick_size.x / 2.,
            -brick_area_size.y / 2. + cfg.brick_size.y / 2. + cfg.brick_area_y_offset,
        );

        for col in 0..cfg.brick_area_cols {
            for row in 0..cfg.brick_area_rows {
                if rng.gen_bool(cfg.brick_spawn_probability) {
                    let brick_offset =
                        Vec2::new(col as f32 * cfg.brick_size.x, row as f32 * cfg.brick_size.y);
                    let translation = (first_brick_translation + brick_offset).extend(cfg.brick_z);
                    let color = cfg
                        .brick_colors
                        .choose(&mut rng)
                        .copied()
                        .unwrap_or(Color::WHITE);
                    commands
                        .spawn(GeometryBuilder::build_as(
                            &shape,
                            DrawMode::Outlined {
                                fill_mode: FillMode::color(color),
                                outline_mode: StrokeMode::color(Color::BLACK),
                            },
                            Transform {
                                translation,
                                ..Default::default()
                            },
                        ))
                        .insert(BallCollider)
                        .insert(Brick)
                        .insert(BoundingBox(cfg.brick_size));
                }
            }
        }
    }
}
