extern crate gl;

use crate::shader::Shader;
use std::ptr;
use gl::types::{GLuint, GLfloat};
use crate::draw::Shape;
use std::rc::Rc;  // Import Rc for reference counting
use crate::vector2d::Vector2D;  // Import Vector2D

pub struct Triangle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,
    vertices: [Vector2D; 3],
    normals: [Vector2D; 3],
}

impl Triangle {
    pub fn new(shader: Rc<Shader>, v1: Vector2D, v2: Vector2D, v3: Vector2D) -> Triangle {
        let edge1 = Vector2D::new(v2.x - v1.x, v2.y - v1.y);
        let edge2 = Vector2D::new(v3.x - v1.x, v3.y - v1.y);
        let normal = Vector2D::new(edge1.y, -edge1.x).normalize();

        let mut triangle = Triangle { 
            vao: 0, 
            vbo: 0, 
            shader,
            vertices: [v1, v2, v3],
            normals: [normal, normal, normal],
        };
        triangle.init();
        triangle
    }
}

impl Shape for Triangle {
    fn init(&mut self) {
        let mut vertices: Vec<GLfloat> = Vec::new();
        for i in 0..3 {
            vertices.extend_from_slice(&[
                self.vertices[i].x, self.vertices[i].y, 0.0,
                self.normals[i].x, self.normals[i].y, 1.0,
            ]);
        }

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

            // Update the stride and offset to match the new vertex shader
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, (3 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    fn draw(&self) {
        unsafe {
            self.shader.use_program();  // Use the shader before drawing
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
}
