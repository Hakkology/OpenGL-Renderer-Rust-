use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowMode};

pub struct GlWindow {
    pub glfw: Glfw,  
    pub window: PWindow,  
    pub events: GlfwReceiver<(f64, WindowEvent)>,  
}

impl GlWindow {
    // Creates a new GlWindow with the specified title and dimensions (width, height)
    pub fn new(title: &str, width: u32, height: u32) -> GlWindow {
        // Initialize the GLFW library, failing if initialization fails
        let mut glfw = glfw::init(fail_on_errors!()).unwrap_or_else(|e| {
            panic!("Failed to initialize GLFW: {:?}", e);
        });

        // Set the window hints to create an OpenGL 3.3 context with the Core profile
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Compat));

        // Create the window and OpenGL context, handling errors if creation fails
        let (mut window, events) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        // Set the window context as the current context for rendering
        window.make_current();
        window.set_key_polling(true);

        // Return the new GlWindow instance
        GlWindow { glfw, window, events }
    }

    // Initializes OpenGL, linking GLFW with OpenGL function loading
    pub fn init_gl(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        
        // Check OpenGL version
        let version = unsafe {
            let version_cstr = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
            version_cstr.to_str().unwrap()
        };
        println!("OpenGL version: {}", version);
        
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);  // Dark teal color
    
            // Set the viewport size to match the window dimensions
            let (width, height) = self.window.get_size();
            gl::Viewport(0, 0, width, height);
        }
    }
    
    // Handles individual window events (e.g., key presses) 
    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            // If the 'Escape' key is pressed, close the window
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true);  // Mark the window to be closed
            }
            _ => {}  // Handle other events here (or ignore them)
        }
    }
}