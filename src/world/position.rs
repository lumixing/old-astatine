use std::ops::Add;

use bevy::prelude::Component;

pub const CHUNK_SIZE: u32 = 64;
pub const TILE_SIZE: u32 = 8;

#[derive(Component, Debug)]
pub struct BlockPos {
    x: u32,
    y: u32
}

#[allow(dead_code)]
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

    pub fn is_relative_chunk_pos(&self) -> bool {
        self.x < CHUNK_SIZE && self.y < CHUNK_SIZE
    }

    pub fn linearize(&self) -> usize {
        (self.x + CHUNK_SIZE * self.y) as usize
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl PartialEq for BlockPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Component, Debug, Hash, Eq, Copy, Clone)]
pub struct ChunkPos {
    x: u32,
    y: u32
}

#[allow(dead_code)]
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

impl PartialEq for ChunkPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for ChunkPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}