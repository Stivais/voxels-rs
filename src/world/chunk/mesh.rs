use std::os::raw::c_void;
use std::ptr;
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

pub fn greedy_mesh(chunk: &mut Chunk) {
    let mut vertices: Vec<i32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut index_offset = 0;

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

                    // Mark all blocks in this quad as visited
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[(x + dx) + (z + dz) * CHUNK_SIZE] = true;
                        }
                    }

                    // Generate vertices and indices for the quad
                    vertices.extend_from_slice(&*quad_vertices(x, y, z, w, d));
                    indices.extend_from_slice(&*generate_block_indices(index_offset));
                    index_offset += 4;
                }
            }
        }
    }
    chunk.mesh = ChunkMesh::create(vertices, indices)
}

fn pack_data(x: u8, y: u8, z: u8, normal: u8, id: u8) -> i32 {
    (x as i32) << 18 |
        (y as i32) << 12 |
        (z as i32) << 6 |
        (normal as i32) << 3 |
        (id as i32)
}

fn quad_vertices(x: usize, y: usize, z: usize, w: usize, h: usize) -> Vec<i32> {
    let (x, y, z, w, h) = (x as u8, y as u8, z as u8, w as u8, h as u8);
    let texture_id: u8 = 0;

    vec![
        pack_data(x, y + 1, z, 4, texture_id),
        pack_data(x, y + 1, z + h, 4, texture_id),
        pack_data(x + w, y + 1, z + h, 4, texture_id),
        pack_data(x + w, y + 1, z, 4, texture_id),
    ]
}

#[inline]
fn generate_block_indices(offset: u32) -> Vec<u32> {
    vec![
        offset, offset + 1, offset + 2, // First triangle
        offset, offset + 2, offset + 3,
    ]
}