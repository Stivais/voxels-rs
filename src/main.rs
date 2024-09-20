extern crate glfw;

use std::cmp::PartialEq;
use std::time::Instant;

use fastnoise_lite::{FastNoiseLite, NoiseType};
use gl::{DEPTH_TEST};
use glfw::{Action, Context, GlfwReceiver, Key, Window, WindowEvent};
use ultraviolet::{Vec3};

use crate::render::camera::Camera;
use crate::render::camera::CameraMovement::{BACKWARD, DOWN, FORWARD, LEFT, RIGHT, UP};
use crate::render::chunk_renderer::{ChunkRenderer};
use crate::render::shaders::Shader;
use crate::render::textures::texture_array::TextureArray;
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
    println!("{}",
         unsafe {
            let mut major = 0;
            let mut minor = 0;
            gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
            format!("OpenGL version: {}.{}", major, minor)
        }
    );

    // uncaps fps
    glfw.set_swap_interval(glfw::SwapInterval::None);


    let mut camera = Camera::create(Vec3::new(0.0, 40.0, 0.0), 90.0, 0.0);

    let mut first_mouse = true;
    let mut last_x: f32 = 1920.0 / 2.0;
    let mut last_y: f32 = 1080.0 / 2.0;

    let mut wireframe = false;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let mut world = World::new();

    let noise = {
        let mut noise = FastNoiseLite::with_seed(8008135);
        noise.set_noise_type(Some(NoiseType::Perlin));
        noise
    };
    make_example_chunks(&mut world, &noise);

    let mut chunk_renderer = unsafe {
        ChunkRenderer::create(
            Shader::new(
                "resources/shader.vert",
                "resources/shader.frag",
            ),
            TextureArray::create(
                vec![
                    "resources/textures/dirt.png",
                    "resources/textures/cobblestone.png",
                ],
                16,
                16,
            ),
        )
    };

    unsafe {
        for (pos, mut chunk) in &mut world.chunks {
            let mut index = 0;
            for vertices in greedy_mesh(&chunk) {
                if vertices.len() == 0 { continue; };
                // NOTE: Might be issue with negative numbers
                let base_instance = ((pos.x  & 0x7FF) << 21) | ((pos.y & 0x7F) << 14) | ((pos.z & 0x7FF) << 3) | index;
                let command = chunk_renderer.get_draw_command(vertices.len() as u32, base_instance as u32);

                chunk_renderer.upload_mesh(&command, vertices);
                chunk.add_draw_command(command);
                index += 1;
            }
        }
    }

    unsafe {
        gl::Enable(DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    // fps metrics
    let mut last_update = Instant::now();
    let mut frame_count = 0;

    while !window.should_close() {
        let now = Instant::now();
        frame_count += 1;

        if now.duration_since(last_update).as_secs_f32() >= 1.0 {
            last_update = now;
            window.set_title(&format!("FPS: {}, press r for wireframe", frame_count));
            frame_count = 0
        }

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

            chunk_renderer.render(&world.chunks, &camera)
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
            WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            }
            WindowEvent::Key(Key::R, _, Action::Press, _) => unsafe {
                toggle_wireframe(wireframe);
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

unsafe fn toggle_wireframe(value: &mut bool) {
    if value == &true {
        *value = false;
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    } else {
        *value = true;
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
}