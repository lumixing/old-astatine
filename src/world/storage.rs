use std::ops::Div;

use bevy::{prelude::*, utils::HashMap, math::ivec2};

use super::chunks::{WORLD_SIZE, CHUNK_SIZE};
// use rand::Rng;

#[derive(Resource, Debug)]
pub struct WorldStorage(pub HashMap<IVec2, ChunkData>);

impl WorldStorage {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        for y in 0..WORLD_SIZE.y {
            for x in 0..WORLD_SIZE.x {
                hashmap.insert(ivec2(x, y), ChunkData::new());
            }
        }
        Self(hashmap)
    }

    pub fn set(&mut self, x: i32, y: i32, i: u32) {
        let chunk_pos = ivec2(x, y).div(CHUNK_SIZE as i32);
        if let Some(chunk_data) = self.0.get_mut(&chunk_pos) {
            let chunk_rel_pos = ivec2(x - chunk_pos.x * CHUNK_SIZE as i32, y - chunk_pos.y * CHUNK_SIZE as i32);
            chunk_data.set_tile(chunk_rel_pos.x, chunk_rel_pos.y, i);
        } else {
            info!("{} -> {} is out of range!", ivec2(x, y), chunk_pos);
        }
    }
}

#[derive(Debug)]
pub struct ChunkData {
    tiles: Vec<u32>,
    // walls: Vec<u32>,
    // light: Vec<f32>,
}

impl ChunkData {
    pub fn new() -> Self {
        // let mut rng = rand::thread_rng();
        Self {
            tiles: vec![0; (CHUNK_SIZE*CHUNK_SIZE) as usize],
            // light: (0..64*64).map(|_| rng.gen()).collect()
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<u32> {
        if self.is_out_of_bounds(x, y) {
            info!("cannot get at ({x},{y}) since it is out of bounds!");
            return None
        }

        let lin = self.linearize(x, y);
        Some(self.tiles[lin])
    }

    pub fn set_tile(&mut self, x: i32, y: i32, i: u32) {
        if self.is_out_of_bounds(x, y) {
            info!("cannot set at ({x},{y}) since it is out of bounds!");
            return;
        }

        let lin = self.linearize(x, y);
        self.tiles[lin] = i;
    }

    // pub fn get_light(&self, x: i32, y: i32) -> Option<f32> {
    //     if self.is_out_of_bounds(x, y) {
    //         info!("cannot get at ({x},{y}) since it is out of bounds!");
    //         return None
    //     }

    //     let lin = self.linearize(x, y);
    //     Some(self.light[lin])
    // }

    pub fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_SIZE as i32
    }

    pub fn linearize(&self, x: i32, y: i32) -> usize {
        (x + CHUNK_SIZE as i32 * y) as usize
    }
}