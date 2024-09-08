extern crate gl;

use crate::shader::Shader;
use std::ptr;
use gl::types::{GLuint, GLfloat};
use crate::draw::Shape;
use std::rc::Rc;  // Import Rc for reference counting
use crate::vector2d::Vector2D;

pub struct Rectangle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,  // Use Rc<Shader>
    top_right: Vector2D,
    bottom_left: Vector2D,
}

impl Rectangle {
    // Yeni bir dikdörtgen oluşturur
    pub fn new(shader: Rc<Shader>, top_right: Vector2D, bottom_left: Vector2D) -> Rectangle {  // Accept Rc<Shader> as input
        let mut rectangle = Rectangle { 
            vao: 0, 
            vbo: 0, 
            shader,
            top_right,
            bottom_left,
        };
        rectangle.init();
        rectangle
    }
}

impl Shape for Rectangle {
    // Dikdörtgeni başlatır ve OpenGL'e yükler
    fn init(&mut self) {
        let vertices: [GLfloat; 12] = [
            self.top_right.x, self.top_right.y, 0.0,
            self.top_right.x, self.bottom_left.y, 0.0,
            self.bottom_left.x, self.bottom_left.y, 0.0,
            self.bottom_left.x, self.top_right.y, 0.0,
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                3 * std::mem::size_of::<GLfloat>() as i32, 
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }
    }

    // Dikdörtgeni çizer
    fn draw(&self) {
        unsafe {
            self.shader.use_program();  // Use the shader before drawing
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Rectangle {
    // Dikdörtgen silindiğinde OpenGL kaynaklarını temizler
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
