// main.rs

extern crate glfw;  // External crate for managing window and OpenGL context
extern crate gl;    // External crate for accessing OpenGL functions

mod window;
mod app;
mod draw;    // Import the draw module
mod triangle;  // Import the triangle module
mod rectangle;
mod circle;
mod shader;

use app::Application;

fn main() {
    // Create a new instance of the Application
    let mut app = Application::new();

    // Initialize OpenGL settings, such as clear color, context setup
    app.init_gl();

    // Start the main loop that handles rendering and input events
    app.run();
}
