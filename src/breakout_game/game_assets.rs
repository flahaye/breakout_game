use bevy::prelude::*;

use super::resources::{GameAssets, GameConfig};

pub(crate) struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_game_assets_system);
    }
}

fn init_game_assets_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cfg: Res<GameConfig>,
) {
    commands.insert_resource(GameAssets {
        score_font: asset_server.load(&cfg.score_font_path),
    });
}
