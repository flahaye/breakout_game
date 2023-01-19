use bevy::prelude::{Color, Vec2};

pub const FRAMERATE: f32 = 60.;

pub const WINDOW_WIDTH: f32 = 430.;
pub const WINDOW_HEIGHT: f32 = 600.;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.21, 0.19, 0.17);

pub const WALL_Z_OFFSET: f32 = 40.;
pub const PADDLE_Z_OFFSET: f32 = 20.;
pub const BALL_Z_OFFSET: f32 = 10.;
pub const BRICK_Z_OFFSET: f32 = 30.;

pub const WALL_THICKNESS: f32 = 5.;

pub const PADDLE_SIZE: Vec2 = Vec2::new(80., 15.);
pub const PADDLE_BASE_SPEED: f32 = 400.;
pub const PADDLE_Y_OFFSET: f32 = 30.;

pub const BALL_RADIUS: f32 = 7.;
pub const BALL_BASE_SPEED: f32 = 550.;
pub const BALL_Y_OFFSET: f32 = 5.;

pub const BRICK_SIZE: Vec2 = Vec2::new(40., 20.);
pub const BRICK_SPAWN_PROBABILITY: f64 = 0.2;
pub const BRICK_COLORS: [Color; 37] = [
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
];

pub const BRICK_AREA_Y_OFFSET: f32 = 100.;
pub const BRICK_AREA_ROWS: u32 = 12;
pub const BRICK_AREA_COLS: u32 = 6;
