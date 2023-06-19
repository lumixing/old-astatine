use std::ops::Div;
use bevy::{prelude::*, utils::HashMap, math::ivec2};
use super::{chunks::{WORLD_SIZE, CHUNK_SIZE, CHUNK_SIZE_I}, blocks::Blocks};

#[derive(Resource, Debug)]
pub struct WorldStorage(HashMap<IVec2, ChunkData>);

#[allow(dead_code)]
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

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Blocks> {
        let chunk_pos = ivec2(x, y).div(CHUNK_SIZE_I);
        let Some(chunk_data) = self.get_chunk_data(chunk_pos) else { return None; };
        let chunk_rel_pos = ivec2(x - chunk_pos.x * CHUNK_SIZE_I, y - chunk_pos.y * CHUNK_SIZE_I);
        chunk_data.get_tile(chunk_rel_pos.x, chunk_rel_pos.y)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, block: Blocks) {
        let chunk_pos = ivec2(x, y).div(CHUNK_SIZE_I);
        let Some(chunk_data) = self.get_mut_chunk_data(chunk_pos) else { return; };
        let chunk_rel_pos = ivec2(x - chunk_pos.x * CHUNK_SIZE_I, y - chunk_pos.y * CHUNK_SIZE_I);
        chunk_data.set_tile(chunk_rel_pos.x, chunk_rel_pos.y, block);
    }

    pub fn get_chunk_data(&self, chunk_pos: IVec2) -> Option<&ChunkData> {
        self.0.get(&chunk_pos)
    }

    pub fn get_mut_chunk_data(&mut self, chunk_pos: IVec2) -> Option<&mut ChunkData> {
        self.0.get_mut(&chunk_pos)
    }

    pub fn linearize(&self, pos: IVec2) -> usize {
        (pos.x + CHUNK_SIZE_I * pos.y) as usize
    }
}

#[derive(Debug)]
pub struct ChunkData {
    tiles: Vec<u32>
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            tiles: vec![0; (CHUNK_SIZE*CHUNK_SIZE) as usize]
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Blocks> {
        if self.is_out_of_bounds(x, y) { return None; };
        let lin = self.linearize(x, y);
        Some(Blocks::from(self.tiles[lin]))
    }

    pub fn set_tile(&mut self, x: i32, y: i32, block: Blocks) {
        if self.is_out_of_bounds(x, y) { return; };
        let lin = self.linearize(x, y);
        self.tiles[lin] = block as u32;
    }

    pub fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= CHUNK_SIZE_I || y >= CHUNK_SIZE_I
    }

    pub fn linearize(&self, x: i32, y: i32) -> usize {
        (x + CHUNK_SIZE_I * y) as usize
    }
}