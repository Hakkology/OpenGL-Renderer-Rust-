use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    // Yeni bir 3D vektör oluşturur
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }

    // Sıfır vektörü oluşturur
    pub fn zero() -> Self {
        Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }

    // Vektörün büyüklüğünü hesaplar
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Vektörü normalleştirir
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag != 0.0 {
            Vector3D {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }

    // İki vektörün nokta çarpımını hesaplar
    pub fn dot(&self, other: &Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // İki vektörün çapraz çarpımını hesaplar
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Vektör toplama işlemi
impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vektör çıkarma işlemi
impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Vektörü skaler ile çarpma işlemi
impl Mul<f32> for Vector3D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Vektörü skaler ile bölme işlemi
impl Div<f32> for Vector3D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar != 0.0 {
            Vector3D {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        } else {
            panic!("Sıfıra bölme hatası")
        }
    }
}