use std::ffi::{CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use gl::types::{GLchar, GLint};
use ultraviolet::{Mat4, Vec3};

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        let mut vert_shader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("failed to open {}", vertex_path));
        let mut frag_shader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("failed to open {}", fragment_path));

        let mut vertex_code = String::new();
        let mut fragment_code = String::new();

        vert_shader_file.read_to_string(&mut vertex_code).expect("Failed to read vertex shader");
        frag_shader_file.read_to_string(&mut fragment_code).expect("Failed to read fragment shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // compile
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compilation_errors(vertex, "VERTEX");

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compilation_errors(fragment, "FRAGMENT");

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compilation_errors(id, "PROGRAM");
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id
        }

        shader
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        let cstr = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.id, cstr.as_ptr()), value);
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &Mat4) {
        let cstr = CString::new(name).unwrap();
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, cstr.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    pub unsafe fn set_vec3(&self, name: &str, vec: &Vec3) {
        let cstr = CString::new(name).unwrap();
        gl::Uniform3f(gl::GetUniformLocation(self.id, cstr.as_ptr()), vec.x, vec.y, vec.z);
    }


    unsafe fn check_compilation_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                    -- --------------------------------------------------- -- ",
                    type_,
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                    -- --------------------------------------------------- -- ",
                    type_,
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}