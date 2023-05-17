use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::{states::GameState, player::player::{Player, Velocity}};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_system(ui_example.in_set(OnUpdate(GameState::InGame)));
    }
}

fn ui_example(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &Velocity), With<Player>>
) {
    let (transform, velocity) = player_query.single();

    egui::Window::new("astatine debug shit").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("position: {}", transform.translation));
        ui.label(format!("velocity: {}", velocity.0));
    });
}