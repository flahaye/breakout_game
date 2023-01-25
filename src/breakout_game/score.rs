//! Score related stuff.

use bevy::prelude::*;

use crate::breakout_game::components::ScoreText;

use super::{components::Score, resources::BreakoutConfig, GameStage};

/// Score logic as a Bevy’s plugin. (see the game rules)
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_score)
            .add_system_to_stage(GameStage::Ui, update_score_system);
    }
}

fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>, cfg: Res<BreakoutConfig>) {
    commands.spawn(Score(cfg.startup_score));

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load(&cfg.score_font_path),
                    font_size: cfg.score_font_size,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(&cfg.score_font_path),
                font_size: cfg.score_font_size,
                color: Color::GOLD,
            }),
        ]),
        ScoreText { section: 1 },
    ));
}

fn update_score_system(
    score_query: Query<&Score>,
    mut score_text_query: Query<(&mut Text, &ScoreText)>,
) {
    if let Ok(score) = score_query.get_single() {
        for (mut text, score_text) in score_text_query.iter_mut() {
            text.sections[score_text.section].value = score.0.to_string();
        }
    }
}
