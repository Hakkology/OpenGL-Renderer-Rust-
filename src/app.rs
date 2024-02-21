use crate::window::GlWindow;

pub struct Application {
    window: GlWindow,
}

impl Application {
    pub fn new() -> Application {
        // Initialize the GlWindow here
        let window = GlWindow::new("Interesting", 800, 600);
        Application { window }
    }

    pub fn init_gl(&mut self) {
        // Delegate OpenGL initialization to the window
        self.window.init_gl();
    }

    pub fn run(&mut self) {
        // Start the main loop of the window
        self.window.main_loop();
    }
}
