pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod materials;
pub mod point;
pub mod ray;
pub mod shapes;
pub mod vec3;

const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_float() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn random_float_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
