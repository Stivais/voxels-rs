use std::cmp::PartialEq;
use ultraviolet::Vec3;
use crate::render::chunk_renderer::DrawElementsIndirectCommand;

/// Default chunk size
pub const CS: usize = 32;
pub const CS_I32: i32 = CS as i32;
pub const CS_F32: f32 = CS as f32;

pub struct Chunk {
    pub blocks: Vec<Block>,
    pub draw_commands: Vec<DrawElementsIndirectCommand>
}

impl Chunk {
    pub fn create(blocks: Vec<Block>) -> Chunk {
        Chunk {
            blocks,
            draw_commands: Vec::with_capacity(6),
        }
    }

    pub fn add_draw_command(&mut self, draw_command: DrawElementsIndirectCommand) {
        self.draw_commands.push(draw_command)
    }

    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        if x < 0 || x >= CS || y < 0 || y >= CS || z < 0 || z >= CS {
            return None;
        }
        let index = x + y * CS + z * CS * CS;
        self.blocks.get(index)
    }

    pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        if x > CS - 1 || y > CS - 1 || z > CS - 1 {
            return true;
        }
        match self.get_block_at(x, y, z) {
            None => { true } // assume its true automatically
            Some(block) => { block.block_type == BlockType::AIR }
        }
    }
}

#[repr(C)]
#[derive(Hash, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x as f32,
            self.y as f32,
            self.z as f32,
        )
    }

    pub fn world_pos(&self) -> Vec3 {
        self.to_vec3() * CS_F32
    }
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