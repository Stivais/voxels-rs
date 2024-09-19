use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr;
use gl::{DRAW_INDIRECT_BUFFER, DYNAMIC_DRAW, DYNAMIC_STORAGE_BIT, ELEMENT_ARRAY_BUFFER, SHADER_STORAGE_BUFFER, TEXTURE_2D_ARRAY, TRIANGLES, UNSIGNED_INT};
use gl::types::{GLintptr, GLsizei, GLsizeiptr};
use ultraviolet::projection::perspective_gl;
use ultraviolet::Vec3;
use crate::render::camera::Camera;
use crate::render::frustum::Frustum;
use crate::render::shaders::Shader;
use crate::render::textures::texture_array::TextureArray;
use crate::world::chunk::chunk::{Chunk, CS, ChunkPosition, CS_F32};

const BUFFER_SIZE: u32 = 500_000_000;
const MAX_DRAW_COMMANDS: usize = 100_000;
const QUAD_SIZE_BYTES: u32 = 8;


#[repr(C)]
#[derive(Clone)]
pub struct DrawElementsIndirectCommand {
    index_count: u32,
    instance_count: u32, // 1
    first_index: u32, // 0
    base_quad: u32,
    base_instance: u32,
}

struct BufferSlot {
    start_bytes: u32,
    size_byes: u32,
}

pub struct ChunkRenderer {
    vao: u32,
    ibo: u32,
    ssbo: u32,
    command_buffer: u32,

    allocation_end: u32,
    used_slots: Vec<BufferSlot>,
    draw_commands: Vec<DrawElementsIndirectCommand>,

    shader: Shader,
    texture_array: TextureArray,
}

impl ChunkRenderer {
    pub unsafe fn create(shader: Shader, texture_array: TextureArray) -> ChunkRenderer {
        let mut renderer = ChunkRenderer {
            vao: 0,
            ibo: 0,
            ssbo: 0,
            command_buffer: 0,
            allocation_end: 0,
            used_slots: vec![],
            draw_commands: vec![],
            shader,
            texture_array
        };

        gl::GenVertexArrays(1, &mut renderer.vao);
        gl::BindVertexArray(renderer.vao);

        gl::GenBuffers(1, &mut renderer.command_buffer);
        gl::GenBuffers(1, &mut renderer.ssbo);

        gl::BindBuffer(SHADER_STORAGE_BUFFER, renderer.ssbo);
        gl::BufferStorage(SHADER_STORAGE_BUFFER, BUFFER_SIZE as GLsizeiptr, ptr::null(), DYNAMIC_STORAGE_BIT);

        gl::GenBuffers(1, &mut renderer.ibo);

        let max_quads = CS * CS * CS * 6;
        let mut indices = Vec::with_capacity(max_quads);
        for i in 0..max_quads as u32 {
            indices.push((i << 2) | 2);
            indices.push((i << 2) | 0);
            indices.push((i << 2) | 1);
            indices.push((i << 2) | 1);
            indices.push((i << 2) | 3);
            indices.push((i << 2) | 2);
        }
        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, renderer.ibo);
        let size = (indices.len() * size_of::<u32>()) as GLsizeiptr;
        let data = &indices[0] as *const u32 as *const c_void;
        gl::BufferData(ELEMENT_ARRAY_BUFFER, size, data, DYNAMIC_DRAW);

        gl::BindBuffer(DRAW_INDIRECT_BUFFER, renderer.command_buffer);
        gl::BufferData(DRAW_INDIRECT_BUFFER, (MAX_DRAW_COMMANDS * size_of::<DrawElementsIndirectCommand>()) as GLsizeiptr, ptr::null(), DYNAMIC_DRAW);

        renderer
    }

    pub unsafe fn render_new(&mut self, chunks: &HashMap<ChunkPosition, Chunk>, camera: &Camera) {
        self.shader.use_program();
        self.texture_array.bind(TEXTURE_2D_ARRAY);
        self.shader.set_int("textureArray", 0);

        let view_projection = perspective_gl(45f32.to_radians(), 1920.0 / 1080.0, 0.1, 10000.0) * camera.view_matrix();
        self.shader.set_mat4("view_projection", &view_projection);
        let frustum = Frustum::create(view_projection);

        for (pos, chunk) in chunks {
            let pos = pos.to_vec3();
            let world_pos = pos * CS_F32;

            // todo: gpu frustum and occlusion culling
            if !frustum.test_aabb(world_pos, world_pos + Vec3::broadcast(CS_F32)) {
                continue;
            }

            let mut index = 0;
            for command in &chunk.draw_commands {
                match index {
                    0 => {
                        if (camera.position.y / CS_F32).floor() >= pos.y {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    1 => {
                        if (camera.position.y / CS_F32).floor() <= pos.y {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    2 => {
                        if (camera.position.x / CS_F32).floor() >= pos.x {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    3 => {
                        if (camera.position.x / CS_F32).floor() <= pos.x {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    4 => {
                        if (camera.position.z / CS_F32).floor() <= pos.z {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    5 => {
                        if (camera.position.z / CS_F32).floor() >= pos.z {
                            self.draw_commands.push(command.clone());
                        }
                    }
                    _ => {}
                }
                index += 1;
            }
        }

        let command_amount = self.draw_commands.len();

        if command_amount == 0 {
            return;
        }

        gl::BindVertexArray(self.vao);

        gl::BindBuffer(DRAW_INDIRECT_BUFFER, self.command_buffer);
        let size = (command_amount * size_of::<DrawElementsIndirectCommand>()) as GLsizeiptr;
        let data = self.draw_commands.as_ptr() as *const c_void;
        gl::BufferData(DRAW_INDIRECT_BUFFER, size, data, DYNAMIC_DRAW);

        gl::BindVertexArray(self.vao);
        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, self.ibo);
        gl::BindBufferBase(SHADER_STORAGE_BUFFER, 0, self.ssbo);

        gl::MultiDrawElementsIndirect(
            TRIANGLES,
            UNSIGNED_INT,
            ptr::null(),
            command_amount as GLsizei,
            0
        );

        gl::BindBufferBase(SHADER_STORAGE_BUFFER, 0, 0);
        gl::BindBuffer(ELEMENT_ARRAY_BUFFER, 0);
        gl::BindBuffer(DRAW_INDIRECT_BUFFER, 0);
        gl::BindVertexArray(0);

        self.draw_commands.clear()
    }

    pub unsafe fn get_draw_command(&mut self, quad_count: u32, base_instance: u32) -> DrawElementsIndirectCommand {
        let requested_size = quad_count * QUAD_SIZE_BYTES;

        // todo: best fit algorithm if buffer is full
        if (BUFFER_SIZE - self.allocation_end) < requested_size {
            panic!();
        }

        let slot = BufferSlot {
            start_bytes: self.allocation_end,
            size_byes: requested_size,
        };
        let cmd = create_command(&slot, base_instance);
        self.used_slots.push(slot);
        self.allocation_end += requested_size;

        cmd
    }

    pub unsafe fn upload_mesh(&self, command: &DrawElementsIndirectCommand, vertices: Vec<u64>) {
        gl::BindBuffer(SHADER_STORAGE_BUFFER, self.ssbo);
        let data = &vertices[0] as *const u64 as *const c_void;
        gl::BufferSubData(
            SHADER_STORAGE_BUFFER,
            ((command.base_quad >> 2) * QUAD_SIZE_BYTES) as GLintptr,
            ((command.index_count / 6) * QUAD_SIZE_BYTES) as GLsizeiptr,
            data
        );
        gl::BindBuffer(SHADER_STORAGE_BUFFER, 0);
    }
}

fn create_command(slot: &BufferSlot, base_instance: u32) -> DrawElementsIndirectCommand {
    DrawElementsIndirectCommand {
        index_count: (slot.size_byes / QUAD_SIZE_BYTES) * 6,
        instance_count: 1,
        first_index: 0,
        base_quad: (slot.start_bytes / QUAD_SIZE_BYTES) << 2,
        base_instance,
    }
}
