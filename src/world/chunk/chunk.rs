use std::cmp::PartialEq;
use crate::world::chunk::mesh::ChunkMesh;

/// Default chunk size
pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    pub blocks: Vec<Block>,
    pub mesh: ChunkMesh,
}

impl Chunk {
    pub fn create(blocks: Vec<Block>) -> Chunk {
        Chunk {
            blocks,
            mesh: ChunkMesh::empty(),
        }
    }

    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        let index = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
        self.blocks.get(index)
    }

    pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        // todo: check neighbouring chunks
        if x + 1 > CHUNK_SIZE || y + 1 > CHUNK_SIZE || z + 1 > CHUNK_SIZE {
            return true;
        }
        match self.get_block_at(x, y, z) {
            None => { true } // assume its true automatically
            Some(block) => { block.block_type == BlockType::AIR }
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub block_type: BlockType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    AIR,
    DIRT,
    COBBLESTONE,
}

// todo: add faces for stuff like grass
fn get_texture_id(block_type: BlockType) -> u32 {
    match block_type {
        BlockType::AIR => { panic!("Should not be possible to be air") } // Should not be possible
        BlockType::DIRT => { 0 }
        BlockType::COBBLESTONE => { 1 }
    }
}