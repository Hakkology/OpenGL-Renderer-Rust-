extern crate gl;

pub trait Shape {
    // Method to initialize vertex data
    fn init(&mut self);

    // Method to draw the shape
    fn draw(&self);
}