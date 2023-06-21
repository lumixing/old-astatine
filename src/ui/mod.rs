use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_debug_lines::*;

use crate::{states::GameState, player::player::Player, world::position::{ChunkPos, BlockPos}};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_plugin(DebugLinesPlugin::default());
        
        app.add_systems((
            ui_example,
        ).in_set(OnUpdate(GameState::InGame)));
    }
}

fn ui_example(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &ChunkPos, &BlockPos), With<Player>>
) {
    let (transform, chunk_pos, tile_pos) = player_query.single();

    egui::Window::new("astatine debug shit").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("world position: {}", transform.translation.xy()));
        ui.label(format!("block position: {:?}", tile_pos));
        ui.label(format!("chunk position: {:?}", chunk_pos));
    });
}