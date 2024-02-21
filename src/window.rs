use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent, WindowMode};

pub struct GlWindow {
    glfw: Glfw,
    window : PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl GlWindow{
    pub fn new (title: &str, width: u32, height: u32) -> GlWindow {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap_or_else(|e| {
            panic!("Failed to initialize GLFW: {:?}", e);
        });

        glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = glfw.create_window(width, height, title, WindowMode::Windowed).
            expect("Failed to create GLFW window.");
        
        window.make_current();
        window.set_key_polling(true);

        GlWindow {glfw, window, events}
    }

    pub fn init_gl(&mut self) {
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
    }

    pub fn main_loop(&mut self){
        while !self.window.should_close() {
            self.glfw.poll_events();
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.window.swap_buffers();
    
            let events: Vec<(f64, WindowEvent)> = glfw::flush_messages(&self.events).collect();
    
            for (_, event) in events {
                self.handle_event(event);
            }
        }
    }

    fn handle_event(&mut self, event: WindowEvent){
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true);
            }
            _ => {}
        }
    }
}