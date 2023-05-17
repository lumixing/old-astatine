mod camera;
pub mod player;

use bevy::prelude::*;
use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            player::spawn,
            camera::spawn
        ).in_schedule(OnEnter(GameState::InGame)));

        app.add_systems((
            player::update_gravity,
            player::update_translation,
            camera::follow_player, 
            camera::mouse
        ).in_set(OnUpdate(GameState::InGame)));
    }
}