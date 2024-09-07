use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use
rand::Rng;
use crate::world::chunk::chunk::{Block, BlockType, Chunk, CHUNK_SIZE, ChunkPosition};

pub struct World {
    pub chunks: HashMap<ChunkPosition, Arc<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
        };

        // world.add_chunk(
        //     ChunkPosition { x: 0, y: 0, z: 0 },
        //     generate_chunk()
        // );
        make_example_chunks(&mut world);

        world
    }

    fn add_chunk(&mut self, chunk_position: ChunkPosition, chunk: Chunk) {
        self.chunks.insert(chunk_position, Arc::new(chunk));
    }
}

pub fn generate_chunk() -> Chunk {
    let start = Instant::now();
    let mut chunk = Chunk::create(vec![Block { block_type: BlockType::AIR }; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]);

    let mut rng = rand::thread_rng();

    // let index = 1 + 1 * CHUNK_SIZE + 1 * CHUNK_SIZE * CHUNK_SIZE;
    // chunk.blocks[index] = Block { block_type: BlockType::COBBLESTONE };
    //
    // let index = 2 + 2 * CHUNK_SIZE + 2 * CHUNK_SIZE * CHUNK_SIZE;
    // chunk.blocks[index] = Block { block_type: BlockType::COBBLESTONE };

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if rng.gen_bool(0.8) {
                    let index = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
                    let block_type: BlockType;

                    if y > CHUNK_SIZE - 2 {
                        block_type = BlockType::DIRT
                    } else {
                        block_type = BlockType::COBBLESTONE
                    }

                    chunk.blocks[index] = Block { block_type };
                }
            }
        }
    }
    chunk.make_mesh();
    println!("chunk init time: {:?}", Instant::now().duration_since(start));
    chunk
}

const CHUNK_AMOUNT: i32 = 1; // multiples by 2

// todo: multi threaded
pub fn make_example_chunks(world: &mut World) {
    for x in -CHUNK_AMOUNT..CHUNK_AMOUNT {
        for z in -CHUNK_AMOUNT..CHUNK_AMOUNT {
            let position = ChunkPosition { x, y: 0, z };
            world.add_chunk(
                position,
                generate_chunk()
            )
        }
    }
}