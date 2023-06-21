use bevy::prelude::*;

use crate::states::GameState;

mod player;
mod camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            player::spawn,
            camera::spawn
        ).in_schedule(OnEnter(GameState::InGame)));
    }
}