use bevy::prelude::*;

pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: "astatine - re-re-rewritten!".to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app
}