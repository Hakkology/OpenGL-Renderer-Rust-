#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    #[allow(dead_code)]
    pub fn zero() -> Self {
        Vector2D { x: 0.0, y: 0.0 }
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != 0.0 {
            Vector2D {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            *self
        }
    }

    #[allow(dead_code)]
    pub fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// ... (rest of the implementations remain the same)