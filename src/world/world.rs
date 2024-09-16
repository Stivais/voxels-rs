use std::collections::HashMap;
use std::sync::Arc;
use fastnoise_lite::FastNoiseLite;
use crate::world::chunk::chunk::{Block, BlockType, Chunk, CHUNK_SIZE, ChunkPosition};
use crate::world::chunk::mesh::greedy_mesh;

pub struct World {
    pub chunks: HashMap<ChunkPosition, Arc<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
        };
        world
    }

    fn add_chunk(&mut self, chunk_position: ChunkPosition, chunk: Chunk) {
        self.chunks.insert(chunk_position, Arc::new(chunk));
    }
}

const CHUNK_AMOUNT: i32 = 32; // multiples by 2

// todo: multi threaded
pub fn make_example_chunks(world: &mut World, noise: &FastNoiseLite) {
    for x in -CHUNK_AMOUNT..CHUNK_AMOUNT {
        for z in -CHUNK_AMOUNT..CHUNK_AMOUNT {
            let position = ChunkPosition { x, y: 0, z };
            world.add_chunk(
                position, generate_chunk_noise(x * CHUNK_SIZE as i32, z * CHUNK_SIZE as i32, noise)
            );
        }
    }
}

fn generate_chunk_noise(chunk_x: i32, chunk_z: i32, noise: &FastNoiseLite) -> Chunk {
    let mut chunk = Chunk::create(vec![Block { block_type: BlockType::AIR }; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]);

    let cs = CHUNK_SIZE as i32;

    for x in 0..cs {
        for z in 0..cs {
            let height = (noise.get_noise_2d((chunk_x + x) as f32, (chunk_z + z) as f32) + 1.0) / 2.0;
            // let y = 0;// (height * 32.0) as i32;
            for y in 0..(height * cs as f32) as i32 {
                let index = (x + y * cs + z * cs * cs) as usize;

                let block_type: BlockType;
                // if y > (CHUNK_SIZE as i32 / 2) {
                //     block_type = BlockType::COBBLESTONE
                // } else {
                    block_type = BlockType::DIRT;
                // }

                chunk.blocks[index] = Block { block_type };
            }
        }
    }
    greedy_mesh(&mut chunk);
    chunk
}