use crate::shader::Shader;
use crate::draw::Shape;
use crate::vector3d::Vector3D;
use std::rc::Rc;
use std::ffi::c_void;
use std::mem;

pub struct Cube {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    shader: Rc<Shader>,
    pub rotation: f32,
    bottom_left_back: Vector3D,
    top_right_front: Vector3D,
}

impl Cube {
    // Yeni bir küp oluşturur
    pub fn new(shader: Rc<Shader>, bottom_left_back: Vector3D, top_right_front: Vector3D) -> Self {
        let mut cube = Cube {
            vao: 0,
            vbo: 0,
            shader,
            rotation: 0.0,
            bottom_left_back,
            top_right_front,
        };
        cube.init();
        cube
    }

    // Küp köşe noktalarını oluşturur
    fn generate_vertices(&self) -> [f32; 108] {
        let blb = self.bottom_left_back;
        let trf = self.top_right_front;

        [
            blb.x, blb.y, trf.z, trf.x, blb.y, trf.z, trf.x, trf.y, trf.z,
            trf.x, trf.y, trf.z, blb.x, trf.y, trf.z, blb.x, blb.y, trf.z,
            trf.x, blb.y, blb.z, blb.x, blb.y, blb.z, blb.x, trf.y, blb.z,
            blb.x, trf.y, blb.z, trf.x, trf.y, blb.z, trf.x, blb.y, blb.z,
            blb.x, trf.y, trf.z, blb.x, trf.y, blb.z, blb.x, blb.y, blb.z,
            blb.x, blb.y, blb.z, blb.x, blb.y, trf.z, blb.x, trf.y, trf.z,
            trf.x, trf.y, trf.z, trf.x, trf.y, blb.z, trf.x, blb.y, blb.z,
            trf.x, blb.y, blb.z, trf.x, blb.y, trf.z, trf.x, trf.y, trf.z,
            blb.x, trf.y, blb.z, trf.x, trf.y, blb.z, trf.x, trf.y, trf.z,
            trf.x, trf.y, trf.z, blb.x, trf.y, trf.z, blb.x, trf.y, blb.z,
            blb.x, blb.y, blb.z, trf.x, blb.y, blb.z, trf.x, blb.y, trf.z,
            trf.x, blb.y, trf.z, blb.x, blb.y, trf.z, blb.x, blb.y, blb.z
        ]
    }
}

impl Shape for Cube {
    // Küpü başlatır
    fn init(&mut self) {
        let vertices = self.generate_vertices();

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    // Küpü çizer
    fn draw(&self) {
        unsafe {
            self.shader.use_program();
            
            let cos_x = f32::cos(self.rotation * 0.3);
            let sin_x = f32::sin(self.rotation * 0.3);
            let cos_y = f32::cos(self.rotation * 0.3);
            let sin_y = f32::sin(self.rotation * 0.3);
            let cos_z = f32::cos(self.rotation * 0.3);
            let sin_z = f32::sin(self.rotation * 0.3);

            let rotation_matrix = [
                cos_y * cos_z, -cos_x * sin_z + sin_x * sin_y * cos_z, sin_x * sin_z + cos_x * sin_y * cos_z, 0.0,
                cos_y * sin_z, cos_x * cos_z + sin_x * sin_y * sin_z, -sin_x * cos_z + cos_x * sin_y * sin_z, 0.0,
                -sin_y, sin_x * cos_y, cos_x * cos_y, 0.0,
                0.0, 0.0, 0.0, 1.0
            ];

            let rotation_loc = gl::GetUniformLocation(self.shader.id(), b"rotation\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(rotation_loc, 1, gl::FALSE, rotation_matrix.as_ptr());

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Cube {
    // Küp silindiğinde OpenGL kaynaklarını temizler
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}