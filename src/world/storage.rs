use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

use super::{position::{ChunkPos, CHUNK_SIZE, BlockPos}, chunks::WORLD_SIZE};

#[derive(Resource)]
pub struct WorldStorage(HashMap<ChunkPos, ChunkData>);

#[allow(dead_code)]
impl WorldStorage {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        for y in 0..WORLD_SIZE.y {
            for x in 0..WORLD_SIZE.x {
                let chunk_pos = ChunkPos::new(x as u32, y as u32);
                hashmap.insert(chunk_pos, ChunkData::new());
            }
        }
        Self(hashmap)
    }

    pub fn get_chunk_data(&self, chunk_pos: ChunkPos) -> Option<&ChunkData> {
        self.0.get(&chunk_pos)
    }
}

pub struct ChunkData {
    blocks: Vec<u32>
}

impl ChunkData {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            // blocks: vec![0; (CHUNK_SIZE*CHUNK_SIZE) as usize]
            blocks: (0..CHUNK_SIZE*CHUNK_SIZE).map(|_| rng.gen_range(0..2)).collect()
        }
    }

    pub fn get_tile(&self, block_pos: BlockPos) -> Option<u32> {
        if !block_pos.is_relative_chunk_pos() { return None; };
        let lin = block_pos.linearize();
        Some(self.blocks[lin])
    }
}