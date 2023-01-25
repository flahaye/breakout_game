//! All the resources used by the game.

use bevy::{
    prelude::{Color, Resource, Vec2},
    time::Timer,
};

/// Configuration used by the game.
///
/// Use the same definition for pixel than Bevy.
#[derive(Resource, Clone)]
pub struct BreakoutConfig {
    /// Expected framerate.
    pub framerate: f32,

    /// Window’s width in pixels.
    pub window_width: f32,

    /// Window’s height in pixels.
    pub window_height: f32,

    /// Window’s background color.
    pub background_color: Color,

    /// Ball position along Z-axis in pixels.
    pub ball_z: f32,
    /// Bricks position along Z-axis in pixels.
    pub brick_z: f32,
    /// Paddle position along Z-axis in pixels.
    pub paddle_z: f32,
    /// Walls position along Z-axis in pixels.
    pub wall_z: f32,

    /// Wall’s thickness in pixels.
    pub wall_thickness: f32,

    /// Paddle’s size in pixels.
    pub paddle_size: Vec2,
    /// Paddle’s speed in pixels/second.
    pub paddle_base_speed: f32,
    /// Paddle’s position along Y-axis as an offset from the bottom of the window in pixels.
    pub paddle_y_offset: f32,

    /// Ball’s radius in pixels.
    pub ball_radius: f32,
    /// Ball’s speed in pixels/second.
    pub ball_base_speed: f32,
    /// Ball’s position along Y-axis as an offset from the paddle in pixels.
    /// Zero means the ball start on the paddle.
    pub ball_y_offset: f32,

    /// Brick’s size in pixels.
    pub brick_size: Vec2,
    /// Probability of a brick to spawn, should be in range `0.0..=1.0`.
    pub brick_spawn_probability: f64,

    /// Brick’s spawning area position along Y-axis as an offset from the center of the window in pixels.
    pub brick_area_y_offset: f32,
    /// Number of rows of the brick’s spawning area.
    pub brick_area_rows: u32,
    /// Number of columns of the brick’s spawning area.
    pub brick_area_cols: u32,

    /// Initial score when the game start.
    pub startup_score: i32,
    /// Score gain for breaking a brick.
    pub score_brick: i32,
    /// Path to the font file used to render score in game.
    pub score_font_path: String,
    /// Font size of the in game score text.
    pub score_font_size: f32,

    /// Possibles colors for bricks, chosen at random.
    pub brick_colors: Vec<Color>,
}

impl Default for BreakoutConfig {
    fn default() -> BreakoutConfig {
        BreakoutConfig {
            framerate: 60.,

            window_width: 430.,
            window_height: 600.,
            background_color: Color::rgb(0.21, 0.19, 0.17),

            ball_z: 1.,
            brick_z: 3.,
            paddle_z: 2.,
            wall_z: 4.,

            wall_thickness: 5.,

            paddle_size: Vec2::new(80., 15.),
            paddle_base_speed: 400.,
            paddle_y_offset: 30.,

            ball_radius: 7.,
            ball_base_speed: 550.,
            ball_y_offset: 5.,

            brick_size: Vec2::new(40., 20.),
            brick_spawn_probability: 0.2,
            brick_area_y_offset: 100.,
            brick_area_rows: 12,
            brick_area_cols: 6,

            startup_score: 0,
            score_brick: 1,
            score_font_path: "fonts/arial.ttf".to_string(),
            score_font_size: 32.,

            brick_colors: vec![
                Color::ALICE_BLUE,
                Color::ANTIQUE_WHITE,
                Color::AQUAMARINE,
                Color::AZURE,
                Color::BEIGE,
                Color::BISQUE,
                Color::BLACK,
                Color::BLUE,
                Color::CRIMSON,
                Color::CYAN,
                Color::DARK_GRAY,
                Color::DARK_GREEN,
                Color::FUCHSIA,
                Color::GOLD,
                Color::GRAY,
                Color::GREEN,
                Color::INDIGO,
                Color::LIME_GREEN,
                Color::MAROON,
                Color::MIDNIGHT_BLUE,
                Color::NAVY,
                // Color::NONE,
                Color::OLIVE,
                Color::ORANGE,
                Color::ORANGE_RED,
                Color::PINK,
                Color::PURPLE,
                Color::RED,
                Color::SALMON,
                Color::SEA_GREEN,
                Color::SILVER,
                Color::TEAL,
                Color::TOMATO,
                Color::TURQUOISE,
                Color::VIOLET,
                Color::WHITE,
                Color::YELLOW,
                Color::YELLOW_GREEN,
            ],
        }
    }
}

/// Indicate if bricks should be spawned or not.
#[derive(Resource)]
pub struct BrickRespawn {
    /// If true, brick will be spawned on the next update, regardless of the timer.
    pub immediate_spawn: bool,

    /// Bricks will spawn at the end of timer.
    /// Timer is reset and pause after a spawn.
    pub timer: Timer,
}
