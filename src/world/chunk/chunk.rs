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
        match self.get_block_at(x, y, z) {
            None => { true } // assume its true automatically
            Some(block) => { block.block_type == BlockType::AIR }
        }
    }

    // todo: greedy meshing (binary) and file reformat
    pub fn make_mesh(&mut self) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = self.get_block_at(x, y, z).unwrap().block_type;
                    if block != BlockType::AIR {
                        // Check visibility of each face

                        let mut do_stuff = |face: Face| {
                            let face_vertices = generate_face_vertices(x, y, z, face, block);
                            vertices.extend_from_slice(&*face_vertices);

                            let block_indices = generate_block_indices(index_offset);
                            indices.extend_from_slice(&*block_indices);
                            index_offset += 4;
                        };

                        // left face
                        if x == 0 || self.is_air(x - 1, y, z) {
                            do_stuff(Face::Left);
                        }
                        // right face
                        if x == CHUNK_SIZE - 1 || self.is_air(x + 1, y, z) {
                            do_stuff(Face::Right);
                        }
                        // bottom face
                        if y == 0 || self.is_air(x, y - 1, z) {
                            do_stuff(Face::Bottom);
                        }
                        // top face
                        if y == CHUNK_SIZE - 1 || self.is_air(x, y + 1, z) {
                            do_stuff(Face::Top);
                        }
                        // front face
                        if z == 0 || self.is_air(x, y, z - 1) {
                            do_stuff(Face::Front);
                        }
                        // back
                        if z == CHUNK_SIZE - 1 || self.is_air(x, y, z + 1) {
                            do_stuff(Face::Back);
                        }
                    }
                }
            }
        }
        self.mesh = ChunkMesh::create(vertices, indices);
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

// todo: pack data into an ideally i32, but realistically an i64
// total 18, todo: allocate more when need
// x: 6 bits, y: 6 bits z: bits, normal: 3, id: 3
fn generate_face_vertices(
    x: usize,
    y: usize,
    z: usize,
    face: Face,
    block_type: BlockType,
) -> Vec<i32> {
    let texture_id = get_texture_id(block_type) as f32;
    let (x, y, z) = (x as u8, y as u8, z as u8);

    let pack = |x: u8, y: u8, z: u8, normal: u8| -> i32 {
        (x as i32) << 18 |  // Shift x to the most significant 6 bits
        (y as i32) << 12 |  // Shift y to the next 6 bits
        (z as i32) << 6 |   // Shift z to the next 6 bits
        (normal as i32) << 3 | // Shift normal to the next 3 bits
        (texture_id as i32)
    };

    match face {
        Face::Front => vec![
            pack(x, y, z, 0),
            pack(x, y + 1, z, 0),
            pack(x + 1, y + 1, z, 0),
            pack(x + 1, y, z, 0),
        ],
        Face::Back => vec![
            pack(x, y + 1, z + 1, 1),      // 0, 1, 1
            pack(x, y, z + 1, 1),          // 0, 0, 1
            pack(x + 1, y, z + 1, 1),    // 1, 0, 1
            pack(x + 1, y + 1, z + 1, 1),  // 1, 1, 1
        ],
        Face::Left => vec![
            pack(x, y, z + 1, 2),
            pack(x, y + 1, z + 1, 2),
            pack(x, y + 1, z, 2),
            pack(x, y, z, 2),
        ],
        Face::Right => vec![
            pack(x + 1, y, z, 3),
            pack(x + 1, y + 1, z, 3),
            pack(x + 1, y + 1, z + 1, 3),
            pack(x + 1, y, z + 1, 3),
        ],
        Face::Top => vec![
            pack(x, y + 1, z, 4),
            pack(x, y + 1, z + 1, 4),
            pack(x + 1, y + 1, z + 1, 4),
            pack(x + 1, y + 1, z, 4),
        ],
        Face::Bottom => vec![
            pack(x, y, z, 5),
            pack(x + 1, y, z, 5),
            pack(x + 1, y, z + 1, 5),
            pack(x, y, z + 1, 5),
        ],
    }
}

enum Face {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

#[inline]
fn generate_block_indices(offset: u32) -> Vec<u32> {
    vec![
        offset, offset + 1, offset + 2, // First triangle
        offset, offset + 2, offset + 3,
    ]
}