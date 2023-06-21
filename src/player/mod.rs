use bevy::prelude::*;

use crate::states::GameState;

pub mod player;
mod camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            player::spawn,
            camera::spawn
        ).in_schedule(OnEnter(GameState::InGame)));
        app.add_systems((
            player::movement,
            camera::follow_player,
            player::update_positions
        ).in_set(OnUpdate(GameState::InGame)));
    }
}