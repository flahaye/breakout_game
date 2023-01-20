use super::components::{BallCollider, BoundingBox, Brick};
use crate::consts::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_brick).insert_resource(BrickRespawn {
            immediate_spawn: true,
            timer: Timer::from_seconds(1., TimerMode::Once),
        });
    }
}

#[derive(Resource)]
struct BrickRespawn {
    immediate_spawn: bool,
    timer: Timer,
}

fn spawn_brick(
    mut commands: Commands,
    time: Res<Time>,
    mut brick_respawn: ResMut<BrickRespawn>,
    bricks_query: Query<&Brick>,
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
            extents: BRICK_SIZE,
            ..Default::default()
        };

        let brick_area_size = Vec2::new(
            BRICK_AREA_COLS as f32 * BRICK_SIZE.x,
            BRICK_AREA_ROWS as f32 * BRICK_SIZE.y,
        );

        // bottom left brick
        let first_brick_translation = Vec2::new(
            -brick_area_size.x / 2. + BRICK_SIZE.x / 2.,
            -brick_area_size.y / 2. + BRICK_SIZE.y / 2. + BRICK_AREA_Y_OFFSET,
        );

        for col in 0..BRICK_AREA_COLS {
            for row in 0..BRICK_AREA_ROWS {
                if rng.gen_bool(BRICK_SPAWN_PROBABILITY) {
                    let brick_offset =
                        Vec2::new(col as f32 * BRICK_SIZE.x, row as f32 * BRICK_SIZE.y);
                    let translation =
                        (first_brick_translation + brick_offset).extend(BRICK_Z_OFFSET);
                    let color = BRICK_COLORS[rng.gen_range(0..BRICK_COLORS.len())];
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
                        .insert(BoundingBox(BRICK_SIZE));
                }
            }
        }
    }
}
