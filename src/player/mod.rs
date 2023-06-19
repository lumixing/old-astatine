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
            // player::update_gravity,
            // player::check_for_collisions,
            player::movee,
            player::update_translation,
            camera::follow_player, 
            camera::mine
        ).in_set(OnUpdate(GameState::InGame)));
    }
}