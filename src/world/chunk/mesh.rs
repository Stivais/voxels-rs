use std::os::raw::c_void;
use std::time::Instant;

use gl::{DYNAMIC_STORAGE_BIT, SHADER_STORAGE_BUFFER};
use gl::types::GLsizeiptr;

use crate::world::chunk::chunk::{Chunk, CHUNK_SIZE, CS_I};

pub struct ChunkMesh {
    vao: u32,
    ssbo: u32,
    pub vertices_len: i32,
}

impl ChunkMesh {
    pub fn empty() -> ChunkMesh {
        ChunkMesh {
            vao: 0,
            ssbo: 0,
            vertices_len: 0,
        }
    }

    pub fn create(vertices: Vec<u64>) -> ChunkMesh {
        let mut mesh = ChunkMesh::empty();
        unsafe {
            mesh.setup_mesh(vertices)
        }
        mesh
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBufferBase(SHADER_STORAGE_BUFFER, 0, self.ssbo);
        gl::DrawArrays(gl::TRIANGLES, 0, self.vertices_len * 6);
        gl::BindVertexArray(0);
    }

    unsafe fn setup_mesh(&mut self, vertices: Vec<u64>) {

        // let vertices = vec![
        //     quad_vertex(0, 0, 0, 2, 1, 0),
        //     quad_vertex(0, 0, 0, 2, 1,c 1),
        //     quad_vertex(0, 0, 0, 1, 1, 2),
        //     quad_vertex(0, 0, 0, 1, 1, 3),
        //     quad_vertex(0, 0, 0, 2, 1, 4),
        //     quad_vertex(0, 0, 0, 2, 1, 5)
        // ];
        //
        // let lookup = vec![1, -1, -1, 1, 1, 1, -1];
        //
        // let test = vec![
        //   vec![22u32, 11],
        //   vec![26, 62],
        // ];
        //
        // for vertex in &vertices {
        //     let x = (vertex & 0x1Fu64);               // x (5 bits) -> bits 0-4
        //     let y = (vertex >> 5) & 0x1Fu64;          // y (5 bits) -> bits 5-9
        //     let z = (vertex >> 10) & 0x1Fu64;         // z (5 bits) -> bits 10-14
        //     let w = (vertex >> 15) & 0x1Fu32;     // width (5 bits) -> bits 15-19
        //     let h = (vertex >> 20) & 0x1Fu32;    // height (5 bits) -> bits 20-24
        //     let normal = (vertex >> 25) & 0x07u32;    // normal (3 bits) -> bits 25-27
        //
        //     println!("x {}; y {}; z {}; w {}; h {}; normal {}", x, y, z, w, h, normal);
        //
        //     for currVertexID in 0..6 {
        //         let w_multi = (22 >> currVertexID) & 1u32;
        //         let h_multi = (11 >> currVertexID) & 1u32;
        //         let (w_dir, h_dir) = (((normal & 2u32) >> 1) * 2u32, 1u32 + (normal >> 2));
        //
        //         if normal > 3u32 {
        //             println!("wmulti {}; hmulti {}, wdir {}, hdir {}", w_multi, h_multi, w_dir, h_dir);
        //         }
        //
        //         let mut position = vec![x as i32, y as i32, z as i32];
        //         position[w_dir as usize] += (w * w_multi) as i32;
        //         position[h_dir as usize] += (h * h_multi) as i32;
        //
        //         // println!("xyz: {:?}", position)
        //     }
        // }

        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        gl::CreateBuffers(1, &mut self.ssbo);
        let size = (vertices.len() * size_of::<u64>()) as GLsizeiptr;
        let data = &vertices[0] as *const u64 as *const c_void;
        gl::NamedBufferStorage(self.ssbo, size, data, DYNAMIC_STORAGE_BIT);

        gl::BindBufferBase(SHADER_STORAGE_BUFFER, 0, self.ssbo);
        self.vertices_len = vertices.len() as i32;
    }
}

// todo: Reduce code amount
pub fn greedy_mesh(chunk: &mut Chunk) {
    let start = Instant::now();
    let mut vertices: Vec<u64> = Vec::new();

    // right
    for x in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x + 1, y, z)
        }

        for y in 0..CS_I {
            for z in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(z + y * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;

                    while z + w < CS_I && is_visible(chunk, x, y, z + w) && !visited[((z + w) + y * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while y + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[((z + i) + (y + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[((z + dx) + (y + dz) * CS_I) as usize] = true;
                        }
                    }

                    vertices.push(quad_vertex(x, y, z, w, d, 3));
                }
            }
        }
    }

    // left
    for x in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x - 1, y, z)
        }

        for y in 0..CS_I {
            for z in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(z + y * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;

                    while z + w < CS_I && is_visible(chunk, x, y, z + w) && !visited[((z + w) + y * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while y + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x, y + d, z + i) || visited[((z + i) + (y + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for width in 0..w {
                        for depth in 0..d {
                            visited[((z + width) + (y + depth) * CS_I) as usize] = true;
                        }
                    }
                    vertices.push(quad_vertex(x, y, z, w, d, 2));
                }
            }
        }
    }

    // front
    for z in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z - 1)
        }

        for y in 0..CS_I {
            for x in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(x + y * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;


                    while z + w < CS_I && is_visible(chunk, x + w, y, z) && !visited[((x + w) + y * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while y + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[((x + i) + (y + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[((x + dx) + (y + dz) * CS_I) as usize] = true;
                        }
                    }

                    vertices.push(quad_vertex(x, y, z, w, d, 0));
                }
            }
        }
    }

    // back
    for z in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y, z + 1)
        }

        for y in 0..CS_I {
            for x in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(x + y * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS_I && is_visible(chunk, x + w, y, z) && !visited[((x + w) + y * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while y + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x + i, y + d, z) || visited[((x + i) + (y + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }

                    for dx in 0..w {
                        for dz in 0..d {
                            visited[((x + dx) + (y + dz) * CS_I) as usize] = true;
                        }
                    }

                    vertices.push(quad_vertex(x, y, z, w, d, 1));
                }
            }
        }
    }

    // top
    for y in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y + 1, z)
        }

        for x in 0..CS_I {
            for z in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(x + z * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS_I && is_visible(chunk, x + w, y, z) && !visited[((x + w) + z * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while z + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[((x + i) + (z + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[((x + dx) + (z + dz) * CS_I) as usize] = true;
                        }
                    }

                    vertices.push(quad_vertex(x, y, z, w, d, 4));
                }
            }
        }
    }

    // bottom
    for y in 0..CS_I {
        let mut visited = vec![false; CHUNK_SIZE * CHUNK_SIZE];

        fn is_visible(chunk: &Chunk, x: isize, y: isize, z: isize) -> bool {
            !chunk.is_air(x, y, z) && chunk.is_air(x, y - 1, z)
        }

        for x in 0..CS_I {
            for z in 0..CS_I {
                if is_visible(chunk, x, y, z) && !visited[(x + z * CS_I) as usize] {
                    let mut w = 1;
                    let mut d = 1;

                    while x + w < CS_I && is_visible(chunk, x + w, y, z) && !visited[((x + w) + z * CS_I) as usize] {
                        w += 1;
                    }

                    'outer: while z + d < CS_I {
                        for i in 0..w {
                            if chunk.is_air(x + i, y, z + d) || visited[((x + i) + (z + d) * CS_I) as usize] {
                                break 'outer;
                            }
                        }
                        d += 1;
                    }
                    for dx in 0..w {
                        for dz in 0..d {
                            visited[((x + dx) + (z + dz) * CS_I) as usize] = true;
                        }
                    }

                    vertices.push(quad_vertex(x, y, z, w, d, 5));
                }
            }
        }
    }
    chunk.mesh = ChunkMesh::create(vertices);
    println!("{}", Instant::duration_since(&Instant::now(), start).as_micros())
}

fn pack_data(x: u8, y: u8, z: u8, width: u8, height: u8, normal: u8, texture_id: u8) -> u32 {
    // Pack data into a 32-bit integer
    (x as u32) |             // x (5 bits) -> bits 0-4
    ((y as u32) << 5) |      // y (5 bits) -> bits 5-9
    ((z as u32) << 10) |     // z (5 bits) -> bits 10-14
    ((width as u32) << 15) | // width (5 bits) -> bits 15-19
    ((height as u32) << 20) |// height (5 bits) -> bits 20-24
    ((normal as u32) << 25) |// normal (3 bits) -> bits 25-27
    ((texture_id as u32) << 28) // texture_id (4 bits) -> bits 28-31
}

fn pack_data_u64(x: u8, y: u8, z: u8, width: u8, height: u8, normal: u8, texture_id: u8) -> u64 {
    (x as u64) |
    ((y as u64) << 6) |
    ((z as u64) << 12) |
    ((width as u64) << 18) |
    ((height as u64) << 24) |
    ((normal as u64) << 30) |
    ((texture_id as u64) << 33)
}



fn quad_vertex(x: isize, y: isize, z: isize, w: isize, h: isize, normal: u8) -> u64 {
    let (mut x, mut y, mut z, w, h) = (x as u8, y as u8, z as u8, w as u8, h as u8);
    let texture_id: u8 = 0;

    match normal {
        1 => {
            x += w;
            z += 1;
        }
        2 => {
            z += w;
        }
        3 => {
            x += 1;
        }
        4 => {
            y += 1;
        }
        5 => {
            x += w;
        }
        _ => {}
    }

    let data = pack_data_u64(x, y, z, w, h, normal, texture_id);
    // println!("before: {:?}, packed: {}, unpacked: {:?}", (x, y, z, w, h, normal, texture_id), data, unpack_data(data));
    data
}