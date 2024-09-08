use glfw::Context;
use std::rc::Rc;

use crate::window::GlWindow;
use crate::draw::Shape;
use crate::triangle::Triangle;
use crate::rectangle::Rectangle;
use crate::circle::Circle;
use crate::shader::Shader;
use crate::vector2d::Vector2D;
use crate::vector3d::Vector3D;
use crate::cube::Cube;

pub struct Application {
    window: GlWindow,
    shapes: Vec<Box<dyn Shape>>,
    cube: Option<Cube>,
}

impl Application {
    // Yeni bir uygulama oluşturur
    pub fn new() -> Application {
        let window = GlWindow::new("OpenGL Shape Renderer", 800, 600);
        Application { window, shapes: Vec::new(), cube: None }
    }

    // OpenGL'i başlatır ve şekilleri oluşturur
    pub fn init_gl(&mut self) {
        self.window.init_gl();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        // Load different fragment shaders and wrap them in Rc
        let orange_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/orange_fragment_shader.glsl"));
        let red_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/red_fragment_shader.glsl"));
        let gradient_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/gradient_fragment_shader.glsl"));
        let normal_shader = Rc::new(Shader::new("src/Shaders/vertex_shader.glsl", "src/Shaders/normal_fragment_shader.glsl"));
        let cube_shader = Rc::new(Shader::new("src/Shaders/cube_vertex_shader.glsl", "src/Shaders/cube_fragment_shader.glsl"));

        // Initialize shapes with shared Rc<Shader>
        let triangle = Box::new(Triangle::new(
            orange_shader.clone(),
            Vector2D::new(-0.5, 0.6),
            Vector2D::new(0.5, 0.6),
            Vector2D::new(0.0, 0.9)
        ));
        self.shapes.push(triangle);

        let normal_triangle = Box::new(Triangle::new(
            normal_shader.clone(),
            Vector2D::new(-0.9, 0.8),
            Vector2D::new(-0.9, -0.8),
            Vector2D::new(-0.7, 0.0)
        ));
        self.shapes.push(normal_triangle);

        let rectangle = Box::new(Rectangle::new(
            red_shader.clone(),
            Vector2D::new(0.5, -0.5),   // top_right
            Vector2D::new(0.9, -0.9)  // bottom_left
        ));
        self.shapes.push(rectangle);
        
        let circle = Box::new(Circle::new(Vector2D::new(0.75, 0.75), 0.2, gradient_shader.clone()));
        self.shapes.push(circle);

        self.cube = Some(Cube::new(
            cube_shader.clone(),
            Vector3D::new(-0.3, -0.3, -0.3),  // bottom_left_back
            Vector3D::new(0.3, 0.3, 0.3)      // top_right_front
        ));  // Create the cube
    }

    // Ana uygulama döngüsünü çalıştırır
    pub fn run(&mut self) {
        while !self.window.window.should_close() {
            self.window.glfw.poll_events();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            // Update cube rotation
            if let Some(cube) = &mut self.cube {
                cube.rotation += 0.01;
            }

            // Loop over each shape and use its assigned shader before drawing
            for shape in &self.shapes {
                shape.draw();
            }

            // Draw cube
            if let Some(cube) = &self.cube {
                cube.draw();
            }

            self.window.window.swap_buffers();

            let events: Vec<(f64, glfw::WindowEvent)> =
                glfw::flush_messages(&self.window.events).collect();
            for (_, event) in events {
                self.window.handle_event(event);
            }
        }

        self.cleanup();
    }

    // Kaynakları temizler
    pub fn cleanup(&mut self) {
        self.shapes.clear();
        self.cube = None;
    }
}
