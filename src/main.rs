extern crate glfw;
extern crate gl;

mod window;
mod app; // Ensure the app module is declared

use app::Application;

fn main() {
    let mut app = Application::new();
    app.init_gl();
    app.run();
}
