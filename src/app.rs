use glfw::Context;
use std::rc::Rc;

use crate::window::GlWindow;
use crate::draw::Shape;
use crate::triangle::Triangle;
use crate::rectangle::Rectangle;
use crate::circle::Circle;
use crate::shader::Shader;
    
pub struct Application {
    window: GlWindow,
    shapes: Vec<Box<dyn Shape>>,
}

impl Application {
    pub fn new() -> Application {
        let window = GlWindow::new("OpenGL Shape Renderer", 800, 600);
        Application { window, shapes: Vec::new() }
    }


    pub fn init_gl(&mut self) {
        self.window.init_gl();

        // Load different fragment shaders and wrap them in Rc
        let orange_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/orange_fragment_shader.glsl"));
        let red_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/red_fragment_shader.glsl"));

        // Initialize shapes with shared Rc<Shader>
        self.shapes.push(Box::new(Triangle::new(orange_shader.clone())));
        self.shapes.push(Box::new(Rectangle::new(red_shader.clone()))); 
        self.shapes.push(Box::new(Circle::new(red_shader.clone()))); 
    }

    pub fn run(&mut self) {
        while !self.window.window.should_close() {
            self.window.glfw.poll_events();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Loop over each shape and use its assigned shader before drawing
            for shape in &self.shapes {
                shape.draw();
            }

            self.window.window.swap_buffers();

            let events: Vec<(f64, glfw::WindowEvent)> =
                glfw::flush_messages(&self.window.events).collect();
            for (_, event) in events {
                self.window.handle_event(event);
            }
        }

            // Perform cleanup when exiting
        self.cleanup();

    }

    pub fn cleanup(&mut self) {
        self.shapes.clear();
    }
}
