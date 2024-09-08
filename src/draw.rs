extern crate gl;

pub trait Shape {
    // Şekli başlatır
    fn init(&mut self);

    // Şekli çizer
    fn draw(&self);
}