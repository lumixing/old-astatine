use bevy::{prelude::*, math::vec3};

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Gravity(f32);

#[derive(Component)]
pub struct Player;

pub fn spawn(
    mut commands: Commands
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: vec3(100.0, 2000.0, 2.0),
                scale: vec3(8.0, 8.0, 8.0),
                ..default()
            },
            ..default()
        },
        Velocity(Vec2::ZERO),
        Gravity(2.0),
        Player
    ));
}

pub fn update_gravity(
    mut player_query: Query<(&mut Velocity, &Gravity), With<Player>>
) {
    let (mut velocity, gravity) = player_query.single_mut();
    velocity.0.y = -gravity.0;
}

pub fn update_translation(
    mut player_query: Query<(&Velocity, &mut Transform), With<Player>>
) {
    let (velocity, mut transform) = player_query.single_mut();
    transform.translation.x += velocity.0.x;
    transform.translation.y += velocity.0.y;
}