use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use rand::seq::SliceRandom;
use states::GameState;

mod states;
mod player;
mod world;
mod ui;

pub fn app() -> App {
    let mut app = App::new();
    let splashes = ["with a new coat of rust", "somewhat safe code", "will probably perform better"];
    let splash = splashes.choose(&mut rand::thread_rng()).unwrap();
    
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: format!("astatine.rs, pre-pre-pre-alpha, {}", splash.to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_state::<GameState>();
    app.add_loading_state(LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::WorldGeneration));
    
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_plugin(player::PlayerPlugin);
    app.add_plugin(world::WorldPlugin);
    app.add_plugin(ui::UIPlugin);

    app
}