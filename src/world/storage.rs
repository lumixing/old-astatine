use std::ops::Div;

use bevy::{prelude::*, utils::HashMap, math::ivec2};

#[derive(Resource, Debug)]
pub struct WorldStorage(pub HashMap<IVec2, ChunkData>);

impl WorldStorage {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        for y in 0..4 {
            for x in 0..4 {
                hashmap.insert(ivec2(x, y), ChunkData::new());
            }
        }
        Self(hashmap)
    }

    pub fn set(&mut self, x: i32, y: i32, i: u32) {
        let chunk_pos = ivec2(x, y).div(64);
        if let Some(chunk_data) = self.0.get_mut(&chunk_pos) {
            let chunk_rel_pos = ivec2(x - chunk_pos.x * 64, y - chunk_pos.y * 64);
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
    // light: Vec<u32>,
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            tiles: vec![0; 64*64]
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

    pub fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= 64 || y >= 64
    }

    pub fn linearize(&self, x: i32, y: i32) -> usize {
        (x + 64 * y) as usize
    }
}