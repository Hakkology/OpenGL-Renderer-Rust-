extern crate gl;

use crate::shader::Shader;
use std::ptr;
use gl::types::{GLuint, GLfloat};
use crate::draw::Shape;
use std::rc::Rc;  // Import Rc for reference counting

pub struct Rectangle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,  // Use Rc<Shader>
}

impl Rectangle {
    pub fn new(shader: Rc<Shader>) -> Rectangle {  // Accept Rc<Shader> as input
        let mut rectangle = Rectangle { vao: 0, vbo: 0, shader };
        rectangle.init();
        rectangle
    }
}

impl Shape for Rectangle {
    fn init(&mut self) {
        let vertices: [GLfloat; 12] = [
            0.5, 0.0, 0.0,  
            0.5, -0.5, 0.0,  
            -0.5, -0.5, 0.0, 
            -0.5, 0.0, 0.0   
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

    fn draw(&self) {
        unsafe {
            self.shader.use_program();  // Use the shader before drawing
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}
