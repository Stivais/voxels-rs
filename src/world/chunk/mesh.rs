use std::os::raw::c_void;
use std::ptr;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};

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


    // data: x 0.0, y 0.0, z 0.0, uvs 0.0, 0.0 textureID i32



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