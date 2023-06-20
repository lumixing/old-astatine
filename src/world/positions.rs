use bevy::prelude::*;

use super::chunks::{TILE_SIZE, CHUNK_SIZE};

#[derive(Component, Debug)]
pub struct ChunkPos {
    x: u32,
    y: u32
}

impl ChunkPos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_world_pos(x: f32, y: f32) -> Self {
        let block_pos = BlockPos::from_world_pos(x, y);
        Self::from_block_pos(block_pos)
    }

    pub fn from_block_pos(block_pos: BlockPos) -> Self {
        Self {
            x: block_pos.x() / CHUNK_SIZE,
            y: block_pos.y() / CHUNK_SIZE
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

#[derive(Component, Debug)]
pub struct BlockPos {
    x: u32,
    y: u32
}

impl BlockPos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_world_pos(x: f32, y: f32) -> Self {
        Self {
            x: x as u32 / TILE_SIZE,
            y: y as u32 / TILE_SIZE
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}