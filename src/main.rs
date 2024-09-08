extern crate glfw;  
extern crate gl;    

mod window;
mod app;
mod shader;
mod draw;
mod circle;
mod triangle;
mod rectangle;
mod vector2d;
mod vector3d;  // Add this line
mod cube;  // Add this line

use app::Application;

fn main() {
    // Create a new instance of the Application
    let mut app = Application::new();

    // Initialize OpenGL settings, such as clear color, context setup
    app.init_gl();

    // Start the main loop that handles rendering and input events
    app.run();
}
