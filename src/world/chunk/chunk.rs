use std::cmp::PartialEq;
use crate::world::chunk::mesh::ChunkMesh;

/// Default chunk size
pub const CHUNK_SIZE: usize = 48;
pub const CS_I: isize = CHUNK_SIZE as isize;

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

    pub fn get_block_at(&self, x: isize, y: isize, z: isize) -> Option<&Block> {
        if x < 0 || x >= CS_I || y < 0 || y >= CS_I || z < 0 || z >= CS_I {
            return None;
        }
        let index = x + y * CS_I + z * CS_I * CS_I;
        self.blocks.get(index as usize)
    }

    pub fn is_air(&self, x: isize, y: isize, z: isize) -> bool {
        if x > CS_I - 1 || y > CS_I - 1 || z > CS_I - 1 {
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