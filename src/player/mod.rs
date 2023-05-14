mod camera;

use bevy::prelude::*;
use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera::spawn.in_schedule(OnEnter(GameState::InGame)));
        app.add_system(camera::movement.in_set(OnUpdate(GameState::InGame)));
    }
}