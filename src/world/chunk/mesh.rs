use std::os::raw::c_void;
use std::ptr;
use std::time::Instant;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use crate::world::chunk::chunk::{Chunk, CHUNK_SIZE};

pub struct ChunkMesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    pub indices_length: i32,
}

impl ChunkMesh {
    pub fn empty() -> ChunkMesh {
        ChunkMesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            indices_length: 0
        }
    }

    pub fn create(vertices: Vec<i32>, indices: Vec<u32>) -> ChunkMesh {
        let mut mesh = ChunkMesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            indices_length: 0
        };
        unsafe { mesh.setup_mesh(vertices, indices) }
        mesh
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices_length, gl::UNSIGNED_INT, ptr::null());
        gl::BindVertexArray(0);
    }

    unsafe fn setup_mesh(&mut self, vertices: Vec<i32>, indices: Vec<u32>) {
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr;
        let data = &vertices[0] as *const i32 as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        let size = (indices.len() * size_of::<u32>()) as isize;
        let data = &indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let stride = size_of::<i32>() as GLsizei; // stride is the size of a single packed integer

        gl::VertexAttribIPointer(0, 1, gl::INT, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);

        self.indices_length = indices.len() as i32
    }
}

// todo: Reduce code amount
pub fn greedy_mesh(chunk: &mut Chunk) {
    let start = Instant::now();
    let mut vertices: Vec<i32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut index_offset = 0;

    // top
    for y in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y + 1, z)
        }

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[x + z * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CHUNK_SIZE && is_visible(chunk, x + w, y, z) && !visited[(x + w) + z * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while z + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[(x + i) + (z + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (z + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 4));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }

    // bottom
    for y in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y - 1, z)
        }

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[x + z * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CHUNK_SIZE && is_visible(chunk, x + w, y, z) && !visited[(x + w) + z * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while z + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[(x + i) + (z + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (z + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 5));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }

    // left
    for x in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x - 1, y, z)
        }

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[z + y * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;


                    while z + w < CHUNK_SIZE && is_visible(chunk, x, y, z + w) && !visited[(z + w) + y * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while y + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[(z + i) + (y + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(z + dx) + (y + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 2));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }

    // right
    for x in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x + 1, y, z)
        }

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[z + y * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;


                    while z + w < CHUNK_SIZE && is_visible(chunk, x, y, z + w) && !visited[(z + w) + y * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while y + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[(z + i) + (y + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(z + dx) + (y + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 3));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }

    // front
    for z in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z - 1)
        }

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[x + y * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;


                    while z + w < CHUNK_SIZE && is_visible(chunk, x + w, y, z) && !visited[(x + w) + y * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while y + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[(x + i) + (y + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (y + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 0));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }

    // back
    for z in 0..CHUNK_SIZE {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: usize, y: usize, z: usize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z + 1)
        }

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if is_visible(chunk, x, y, z) && !visited[x + y * CHUNK_SIZE] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CHUNK_SIZE && is_visible(chunk, x + w, y, z) && !visited[(x + w) + y * CHUNK_SIZE] {
                        w += 1;
                    }

                    'outer: while y + d < CHUNK_SIZE {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[(x + i) + (y + d) * CHUNK_SIZE] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (y + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d, 1));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }
    chunk.mesh = ChunkMesh::create(vertices, indices);
    println!("{}", Instant::duration_since(&Instant::now(), start).as_micros())
}

fn pack_data(x: u8, y: u8, z: u8, normal: u8, id: u8) -> i32 {
    (x as i32) << 18 |
    (y as i32) << 12 |
    (z as i32) << 6 |
    (normal as i32) << 3 |
    (id as i32)
}

fn quad_vertices(x: usize, y: usize, z: usize, w: usize, h: usize, normal: u8) -> Vec<i32> {
    let (x, y, z, w, h) = (x as u8, y as u8, z as u8, w as u8, h as u8);


    let texture_id: u8 = 0;

    match normal {
        0 => vec![
            pack_data(x, y + h, z, normal, texture_id),        // top-left
            pack_data(x + w, y + h, z, normal, texture_id),    // top-right
            pack_data(x + w, y, z, normal, texture_id),        // bottom-right
            pack_data(x, y, z, normal, texture_id)             // bottom-left
        ],
        1 => vec![
            pack_data(x, y, z + 1, normal, texture_id),            // bottom-left
            pack_data(x + w, y, z + 1, normal, texture_id),        // bottom-right
            pack_data(x + w, y + h, z + 1, normal, texture_id),    // top-right
            pack_data(x, y + h, z + 1, normal, texture_id)         // top-left
        ],
        2 => vec![
            pack_data(x, y, z + w, normal, texture_id),
            pack_data(x, y + h, z + w, normal, texture_id),
            pack_data(x, y + h, z, normal, texture_id),
            pack_data(x, y, z, normal, texture_id),
        ],
        3 => vec![
            pack_data(x + 1, y, z, 3, texture_id),
            pack_data(x + 1, y + h, z, 3, texture_id),
            pack_data(x + 1, y + h, z + w, 3, texture_id),
            pack_data(x + 1, y, z + w, 3, texture_id),
        ],
        4 => vec![
            pack_data(x, y + 1, z, normal, texture_id),
            pack_data(x, y + 1, z + h, normal, texture_id),
            pack_data(x + w, y + 1, z + h, normal, texture_id),
            pack_data(x + w, y + 1, z, normal, texture_id),
        ],
        _ => vec![
            pack_data(x, y, z, normal, texture_id),              // bottom-left
            pack_data(x + w, y, z, normal, texture_id),          // bottom-right
            pack_data(x + w, y, z + h, normal, texture_id),      // top-right
            pack_data(x, y, z + h, normal, texture_id),          // top-left
        ]
    }
}

/*
    match face {
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
    }
*/

#[inline]
fn generate_block_indices(offset: u32) -> Vec<u32> {
    vec![
        offset, offset + 1, offset + 2, // First triangle
        offset, offset + 2, offset + 3,
    ]
}