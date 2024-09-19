use std::collections::HashMap;
use fastnoise_lite::FastNoiseLite;
use crate::world::chunk::chunk::{Block, BlockType, Chunk, CHUNK_SIZE, ChunkPosition, CS_F32, CS_I32};

pub struct World {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
        };
        world
    }

    fn add_chunk(&mut self, chunk_position: ChunkPosition, chunk: Chunk) {
        self.chunks.insert(chunk_position, chunk);
    }
}

const CHUNK_AMOUNT: i32 = 32;

// todo: multi threaded
pub fn make_example_chunks(world: &mut World, noise: &FastNoiseLite) {
    // world.add_chunk(
    //     ChunkPosition { x: 0, y: 0, z: 0 }, generate_chunk_noise(CHUNK_SIZE as i32, CHUNK_SIZE as i32, noise)
    // );

    for x in 0..CHUNK_AMOUNT * 2 {
        for z in 0..CHUNK_AMOUNT * 2 {
            let position = ChunkPosition { x, y: 0, z };
            world.add_chunk(
                position, generate_chunk_noise(x * CHUNK_SIZE as i32, z * CHUNK_SIZE as i32, noise)
            );
        }
    }
}

fn generate_chunk_noise(chunk_x: i32, chunk_z: i32, noise: &FastNoiseLite) -> Chunk {
    let mut chunk = Chunk::create(vec![Block { block_type: BlockType::AIR }; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]);

    for x in 0..CS_I32 {
        for z in 0..CS_I32 {
            let height = (noise.get_noise_2d((chunk_x + x) as f32, (chunk_z + z) as f32) + 1.0) / 2.0;
            // let y = 0;// (height * 32.0) as i32;
            for y in 0..(height * CS_F32) as i32 {
                let index = (x + y * CS_I32 + z * CS_I32 * CS_I32) as usize;

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

    // super flat
    // for x in 0..CS_I32 {
    //     for y in 0..4 {
    //         for z in 0..CS_I32 {
    //             let index = (x + y * CS_I32 + z * CS_I32 * CS_I32) as usize;
    //             chunk.blocks[index] = Block { block_type: BlockType::DIRT };
    //         }
    //     }
    // }
    chunk
}