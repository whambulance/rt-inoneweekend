use std::ops::Mul;

const MULTIPLIER: f64 = 255.999;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    // pub fn new(r: f64, g: f64, b: f64) -> Self {
    //     Self { r, g, b }
    // }

    pub fn write(&self) {
        let rbyte: u32 = (MULTIPLIER * self.r) as u32;
        let gbyte: u32 = (MULTIPLIER * self.g) as u32;
        let bbyte: u32 = (MULTIPLIER * self.b) as u32;

        println!("{} {} {}", rbyte, gbyte, bbyte);
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, f: f64) -> Self {
        Self {
            r: self.r * f,
            g: self.g * f,
            b: self.b * f,
        }
    }
}
