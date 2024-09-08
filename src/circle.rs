extern crate gl;

use crate::shader::Shader;
use std::ptr;
use std::f32::consts::PI;
use gl::types::{GLuint, GLfloat};
use crate::draw::Shape;
use std::rc::Rc;  // Import Rc for reference counting

pub struct Circle {
    vao: GLuint,
    vbo: GLuint,
    num_vertices: i32,
    shader: Rc<Shader>,  // Use Rc<Shader>
}

impl Circle {
    pub fn new(shader: Rc<Shader>) -> Circle {  // Accept Rc<Shader> as input
        let mut circle = Circle {
            vao: 0,
            vbo: 0,
            num_vertices: 0,
            shader,  // Initialize shader
        };
        circle.init();
        circle
    }

    // Helper function to generate circle vertices
    fn generate_circle_vertices(radius: f32, num_segments: usize, center_x: f32, center_y: f32) -> Vec<GLfloat> {
        let mut vertices = Vec::with_capacity((num_segments + 2) * 3);
    
        // Center vertex (the center of the circle)
        vertices.push(center_x);  // X position
        vertices.push(center_y);  // Y position
        vertices.push(0.0);       // Z position
    
        // Circle vertices (edge of the circle)
        for i in 0..=num_segments {
            let theta = 2.0 * PI * i as f32 / num_segments as f32;
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            vertices.push(center_x + x);  // Translate X
            vertices.push(center_y + y);  // Translate Y
            vertices.push(0.0);           // Z position
        }
        vertices
    }
}

impl Shape for Circle {
    fn init(&mut self) {
        let vertices = Circle::generate_circle_vertices(0.25, 40, 0.65, 0.65);
        self.num_vertices = vertices.len() as i32 / 3;  // Number of vertices

        unsafe {
            // Generate buffers and arrays
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            // Bind the Vertex Array Object
            gl::BindVertexArray(self.vao);

            // Bind the Vertex Buffer Object and copy vertex data to the buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Define the vertex attributes (position in this case)
            gl::VertexAttribPointer(
                0,                 // Index of the vertex attribute
                3,                 // Size of the vertex attribute (3 floats for position)
                gl::FLOAT,         // Type of the data
                gl::FALSE,         // Normalized?
                3 * std::mem::size_of::<GLfloat>() as i32, // Stride between vertex data
                ptr::null(),       // Offset to the first component
            );
            gl::EnableVertexAttribArray(0);

            // Unbind the VAO for now
            gl::BindVertexArray(0);
        }
    }

    fn draw(&self) {
        unsafe {
            self.shader.use_program();  // Use the shader before drawing
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
