extern crate gl;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;
use gl::types::*;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    // Create a new shader program by loading, compiling, and linking vertex and fragment shaders
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let vertex_code = Shader::read_shader_source(vertex_path);
        let fragment_code = Shader::read_shader_source(fragment_path);

        let vertex_shader = Shader::compile_shader(&vertex_code, gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(&fragment_code, gl::FRAGMENT_SHADER);

        let program_id = Shader::link_program(vertex_shader, fragment_shader);

        // Clean up shaders after linking
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: program_id }
    }

    // Use the shader program
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    // Utility function to read shader source code from file
    fn read_shader_source(path: &str) -> String {
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut source = String::new();
        file.read_to_string(&mut source).expect("Failed to read shader file");
        source
    }

    // Utility function to compile a shader
    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Check for shader compile errors
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Shader compilation failed: {}", error.to_str().unwrap());
            }
        }
        shader
    }

    // Utility function to link the vertex and fragment shaders into a program
    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // Check for linking errors
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Program linking failed: {}", error.to_str().unwrap());
            }
        }
        program
    }
}
