extern crate glfw;

use std::cmp::PartialEq;
use std::time::Instant;

use fastnoise_lite::{FastNoiseLite, NoiseType};
use gl::{DEPTH_TEST, TEXTURE_2D_ARRAY};
use glfw::{Action, Context, GlfwReceiver, Key, Window, WindowEvent};
use ultraviolet::{Vec3};
use ultraviolet::projection::perspective_gl;

use crate::render::camera::Camera;
use crate::render::camera::CameraMovement::{BACKWARD, DOWN, FORWARD, LEFT, RIGHT, UP};
use crate::render::chunk_renderer::{ChunkRenderer, DrawElementsIndirectCommand};
use crate::render::frustum::{Frustum};
use crate::render::shaders::Shader;
use crate::render::textures::texture_array::TextureArray;
use crate::world::chunk::chunk::{CS_F32};
use crate::world::chunk::mesh::greedy_mesh;
use crate::world::world::{make_example_chunks, World};

mod render;
mod world;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    // #[cfg(target_os = "macos")]
    // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(1920, 1080, "Window", glfw::WindowMode::Windowed).expect("Failed to create");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let version = unsafe {
        let mut major = 0;
        let mut minor = 0;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
        format!("OpenGL version: {}.{}", major, minor)
    };
    println!("{}", version);

    // uncaps fps
    glfw.set_swap_interval(glfw::SwapInterval::None);


    let mut camera = Camera::create(Vec3::new(0.0, 40.0, 0.0), 90.0, 0.0);

    let mut first_mouse = true;
    let mut last_x: f32 = 1920.0 / 2.0;
    let mut last_y: f32 = 1080.0 / 2.0;

    let mut wireframe = false;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;


    let noise = {
        let mut noise = FastNoiseLite::with_seed(8008135);
        noise.set_noise_type(Some(NoiseType::Perlin)); // No need to wrap in Some if unnecessary
        noise
    };

    let mut world = World::new();
    make_example_chunks(&mut world, &noise);

    let mut chunk_renderer = unsafe { ChunkRenderer::create() };

    unsafe {
        for (pos, mut chunk) in &mut world.chunks {
            let mut index = 0;
            for vertices in greedy_mesh(&chunk) {
                if vertices.len() == 0 { continue; };
                // NOTE: Might be issue with negative numbers
                let base_instance = (pos.x) | (pos.y << 10) | (pos.z << 20);
                let command = chunk_renderer.get_draw_command(vertices.len() as u32, base_instance as u32);

                chunk_renderer.upload_mesh(&command, vertices);
                chunk.add_draw_command(command);
                index += 1;
            }
        }
    }

    let shader = Shader::new(
        "resources/shader.vert",
        "resources/shader.frag",
    );

    let texture_array = TextureArray::create(
        vec![
            "resources/textures/dirt.png",
            "resources/textures/cobblestone.png",
        ],
        16,
        16,
    );

    unsafe {
        gl::Enable(DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut draw_commands: Vec<DrawElementsIndirectCommand> = Vec::new();

    // fps metrics
    let mut last_update = Instant::now();
    let mut frame_count = 0;
    let mut chunk_count = 0;
    let mut visible_chunks = 0;

    while !window.should_close() {
        let now = Instant::now();
        frame_count += 1;

        if now.duration_since(last_update).as_secs_f32() >= 1.0 {
            last_update = now;
            window.set_title(&format!("FPS: {}, Chunks: {}, Visible Chunks: {}", frame_count, chunk_count, visible_chunks));
            frame_count = 0
        }
        chunk_count = 0;
        visible_chunks = 0;

        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // input

        process_input(&mut window, &mut camera, delta_time);
        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut camera, &mut wireframe);


        // render

        unsafe {
            gl::ClearColor(0.0, 2.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            texture_array.bind(TEXTURE_2D_ARRAY);

            shader.set_int("textureArray", 0);

            let view_projection = perspective_gl(45.0f32.to_radians(), 1920.0 / 1080.0, 0.1, 10000.0) * camera.view_matrix();
            shader.set_mat4("view_projection", &view_projection);
            let frustum = Frustum::from_matrix(&view_projection); // Frustum::create(&view_projection);//create_frustum_from_camera(&camera, 1920.0 / 1080.0, 45.0f32.to_radians(),0.1, 10000.0);

            let ref cam_pos = camera.position;

            for (pos, chunk) in &world.chunks {

                chunk_count += 1;

                let (chunk_x, chunk_y, chunk_z) = (pos.x as f32 * CS_F32, pos.y as f32 * CS_F32, pos.z as f32 * CS_F32);

                if !frustum.test_aabb(
                    Vec3::new(chunk_x, chunk_y, chunk_z),
                    Vec3::new(chunk_x + CS_F32, chunk_y + CS_F32, chunk_z + CS_F32),
                ) {
                    continue;
                }
                visible_chunks += 1;

                let mut index = 0;

                for command in &chunk.draw_commands {
                    match index {
                        0 => {
                            if (cam_pos.y / CS_F32).floor() >= pos.y as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                        1 => {
                            if (cam_pos.y / CS_F32).floor() <= pos.y as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                        2 => {
                            if (cam_pos.x / CS_F32).floor() >= pos.x as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                        3 => {
                            if (cam_pos.x / CS_F32).floor() <= pos.x as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                        4 => {
                            if (cam_pos.z / CS_F32).floor() <= pos.z as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                        _ => {
                            if (cam_pos.z / CS_F32).floor() >= pos.z as f32 {
                                draw_commands.push(command.clone());
                            }
                        }
                    }
                    index += 1;
                }
            }

            chunk_renderer.render(&draw_commands, &shader, &camera);
            draw_commands.clear()

            // let projection: Mat4 = perspective_gl(45.0f32.to_radians(), 1920.0 / 1080.0, 0.1, 10000.0);
            // shader.set_mat4("projection", &projection);
            // let view: Mat4 = camera.view_matrix();
            // shader.set_mat4("view", &view);CA
            //
            //
            // for (pos, chunk) in &world.chunks {
            //     const SIZE: i32 = CHUNK_SIZE as i32;
            //     let world_positon = Vec3::new((pos.x * SIZE) as f32, (pos.y * SIZE) as f32, (pos.z * SIZE) as f32);
            //     shader.set_vec3("worldPosition", &world_positon);
            //     chunk.mesh.draw();
            // }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut Window, camera: &mut Camera, delta: f32) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }
    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(FORWARD, delta);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(BACKWARD, delta);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(LEFT, delta);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(RIGHT, delta);
    }
    if window.get_key(Key::Space) == Action::Press {
        camera.process_keyboard(UP, delta);
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        camera.process_keyboard(DOWN, delta);
    }
}

pub fn process_events(
    events: &GlfwReceiver<(f64, WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
    wireframe: &mut bool,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height)
                }
            }
            WindowEvent::Key(Key::R, _, Action::Press, _) => unsafe {
                if wireframe == &true {
                    *wireframe = false;
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                } else {
                    *wireframe = true;
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
            }
            WindowEvent::CursorPos(x, y) => {
                let (x, y) = (x as f32, y as f32);
                if *first_mouse {
                    *last_x = x;
                    *last_y = y;
                    *first_mouse = false;
                }

                let x_offset = x - *last_x;
                let y_offset = *last_y - y; // reversed since y-coordinates go from bottom to top

                *last_x = x;
                *last_y = y;

                camera.process_mouse(x_offset, y_offset);
            }
            _ => {}
        }
    }
}