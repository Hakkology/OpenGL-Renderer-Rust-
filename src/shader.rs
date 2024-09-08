extern crate gl;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;
use gl::types::*;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    // Yeni bir shader programı oluşturur
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let vertex_code = Shader::read_shader_source(vertex_path);
        let fragment_code = Shader::read_shader_source(fragment_path);

        let vertex_shader = Shader::compile_shader(&vertex_code, gl::VERTEX_SHADER);
        let fragment_shader = Shader::compile_shader(&fragment_code, gl::FRAGMENT_SHADER);

        let program_id = Shader::link_program(vertex_shader, fragment_shader);

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: program_id }
    }

    // Shader programını kullanır
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    // Shader kaynak kodunu dosyadan okur
    fn read_shader_source(path: &str) -> String {
        let mut file = File::open(path).expect("Shader dosyası açılamadı");
        let mut source = String::new();
        file.read_to_string(&mut source).expect("Shader dosyası okunamadı");
        source
    }

    // Shader'ı derler
    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Shader derleme hatası: {}", error.to_str().unwrap());
            }
        }
        shader
    }

    // Vertex ve fragment shader'ları birleştirir
    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::from_vec_unchecked(vec![b' '; len as usize]);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                panic!("Program bağlama hatası: {}", error.to_str().unwrap());
            }
        }
        program
    }

    // Shader program ID'sini döndürür
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}
