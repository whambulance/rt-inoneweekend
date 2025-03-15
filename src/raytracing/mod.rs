pub mod camera;
pub mod color;
pub mod hittable;
pub mod point;
pub mod ray;
pub mod shapes;
pub mod vec3;

const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
