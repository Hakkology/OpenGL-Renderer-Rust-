extern crate gl;

use crate::shader::Shader;
use crate::draw::Shape;
use crate::vector2d::Vector2D;
use std::rc::Rc;
use std::ffi::c_void;
use std::mem;

pub struct Circle {
    center: Vector2D,
    radius: f32,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    shader: Rc<Shader>,
    num_vertices: i32,
}

impl Circle {
    pub fn new(center: Vector2D, radius: f32, shader: Rc<Shader>) -> Self {
        let mut circle = Circle {
            center,
            radius,
            vao: 0,
            vbo: 0,
            shader,
            num_vertices: 0,
        };
        circle.init();
        circle
    }
}

impl Shape for Circle {
    fn init(&mut self) {
        let (vertices, tex_coords) = Circle::generate_circle_vertices(self.radius, 40, &self.center);
        self.num_vertices = vertices.len() as i32 / 3;

        let mut combined_data = Vec::new();
        for i in 0..self.num_vertices as usize {
            combined_data.extend_from_slice(&vertices[i*3..(i+1)*3]);
            combined_data.extend_from_slice(&tex_coords[i*2..(i+1)*2]);
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (combined_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                combined_data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<f32>() as gl::types::GLsizei,
                (3 * mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn draw(&self) {
        unsafe {
            self.shader.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.num_vertices);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Circle {
    fn drop(&mut self) {
        unsafe {
            // Clean up VAO and VBO
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl Circle {
    fn generate_circle_vertices(radius: f32, segments: i32, center: &Vector2D) -> (Vec<f32>, Vec<f32>) {
        let mut vertices = Vec::new();
        let mut tex_coords = Vec::new();

        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
            let x = center.x + radius * theta.cos();
            let y = center.y + radius * theta.sin();
            vertices.extend_from_slice(&[x, y, 0.0]);
            
            let tx = (theta.cos() + 1.0) / 2.0;
            let ty = (theta.sin() + 1.0) / 2.0;
            tex_coords.extend_from_slice(&[tx, ty]);
        }

        (vertices, tex_coords)
    }
}
