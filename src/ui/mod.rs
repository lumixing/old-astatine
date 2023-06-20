use std::ops::Mul;

use bevy::{prelude::*, math::{Vec3Swizzles, vec3, ivec2}};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_prototype_debug_lines::*;

use crate::{states::GameState, player::player::{Player, Velocity}, world::chunks::{ChunkPos, BlockPos, Collidable}};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_plugin(DebugLinesPlugin::default());
        
        app.add_systems((
            ui_example,
            draw_collideables
        ).in_set(OnUpdate(GameState::InGame)));
    }
}

fn ui_example(
    mut contexts: EguiContexts,
    player_query: Query<(&Transform, &Velocity, &ChunkPos, &BlockPos), With<Player>>
) {
    let (transform, velocity, chunk_pos, tile_pos) = player_query.single();

    egui::Window::new("astatine debug shit").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("world position: {}", transform.translation.xy()));
        ui.label(format!("block position: {}", tile_pos.0));
        ui.label(format!("chunk position: {}", chunk_pos.0));
        ui.label(format!("velocity: {}", velocity.0));
    });
}

fn draw_collideables(
    mut lines: ResMut<DebugLines>,
    query: Query<&TilePos, With<Collidable>>
) {
    info!("drawing {} colls", query.iter().count());
    for tile_pos in query.iter() {
        let pos = ivec2(tile_pos.x as i32, tile_pos.y as i32).mul(8);
        // let pos = tile_pos.0.mul(ivec2(8, 8)).add(ivec2(-4, -4));
        let a = vec3(pos.x as f32, pos.y as f32, 10.0);
        let b = vec3(pos.x as f32 + 8.0, pos.y as f32, 10.0);
        let c = vec3(pos.x as f32, pos.y as f32 + 8.0, 10.0);
        let d = vec3(pos.x as f32 + 8.0, pos.y as f32 + 8.0, 10.0);
        info!("drawing at {a}, {b}, {c}, {d}");
        lines.line_colored(a, b, 0.0, Color::GREEN);
        lines.line_colored(b, d, 0.0, Color::GREEN);
        lines.line_colored(c, d, 0.0, Color::GREEN);
        lines.line_colored(a, c, 0.0, Color::GREEN);
        lines.line_colored(a, d, 0.0, Color::GREEN);
    }
}