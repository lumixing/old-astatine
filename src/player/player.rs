use std::ops::Div;

use bevy::input::mouse::MouseWheel;
use bevy::math::{vec2, Vec3Swizzles, ivec2};
use bevy::{prelude::*, math::vec3};
use bevy::sprite::collide_aabb::{collide};

use crate::world::WorldStorage;
use crate::world::chunks::{self, ChunkPos, TILE_SIZE, CHUNK_SIZE};

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
                translation: vec3(100.0, 100.0, 20.0),
                scale: vec3(8.0, 16.0, 8.0),
                ..default()
            },
            ..default()
        },
        ChunkPos(ivec2(0, 0)),
        Velocity(Vec2::ZERO),
        Gravity(1.0),
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
    mut player_query: Query<(&Velocity, &mut Transform, &mut ChunkPos), With<Player>>
) {
    let (velocity, mut transform, mut chunk_pos) = player_query.single_mut();
    transform.translation.x += velocity.0.x;
    transform.translation.y += velocity.0.y;
    let current_chunk_pos = transform.translation.div((TILE_SIZE * CHUNK_SIZE) as f32).floor().xy().as_ivec2();
    if current_chunk_pos != chunk_pos.0 {
        chunk_pos.0 = current_chunk_pos;
    }
}

pub fn movee(
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut plrq: Query<&mut Transform, With<Player>>,
    mut camq: Query<&mut OrthographicProjection, With<Camera>>,
) {
    // translate camera
    let mut transform = plrq.single_mut();
    let mut projection = camq.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += 5.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.y += 5.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.y -= 5.0;
    }

    // scroll zoom camera
    for ev in scroll_evr.iter() {
        projection.scale -= ev.y * 0.1;
    }
}

// pub fn check_for_collisions(
//     mut player_query: Query<(&Transform, &mut Velocity), With<Player>>,
//     tile_query: Query<&GlobalTilePos, With<Collidable>>
// ) {
//     let (player_transform, mut player_velocity) = player_query.single_mut();
//     for tile_pos in tile_query.iter() {
//         let collision = collide(
//             player_transform.translation, 
//             player_transform.scale.truncate(),
//             vec3(tile_pos.0.x as f32 * 8.0, tile_pos.0.y as f32 * 8.0, player_transform.translation.z),
//             vec2(8.0, 8.0)
//         );

//         if let Some(_did_collide) = collision {
//             info!("COLLIDE!!");
//             player_velocity.0 = Vec2::ZERO;
//         } else {
//             // info!("nofin :(");
//         }
//     }
// }

// pub fn create_colls(
//     player_query: Query<&Transform, With<Player>>,
//     mut commands: Commands,
//     rendered_chunks: Res<RenderedChunks>,
//     world_storage: Res<WorldStorage>,
// ) {
//     let player_transform = player_query.single();
//     chunks::make_coll(
//         player_transform.translation.xy(),
//         commands,
//         rendered_chunks,
//         world_storage
//     );
// }