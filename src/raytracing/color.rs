use std::ops::{AddAssign, Mul, MulAssign};

use crate::raytracing::interval::Interval;

use super::{random_float, random_float_range};

const MULTIPLIER: f64 = 255.999;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    pub fn write(&self) {
        const INTENSITY: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let r = Self::linear_to_gamma(self.r);
        let g = Self::linear_to_gamma(self.g);
        let b = Self::linear_to_gamma(self.b);
        // let rbyte: u32 = (MULTIPLIER * self.r) as u32;
        // let gbyte: u32 = (MULTIPLIER * self.g) as u32;
        // let bbyte: u32 = (MULTIPLIER * self.b) as u32;

        let rbyte = (256.0 * INTENSITY.clamp(r)) as u32;
        let gbyte = (256.0 * INTENSITY.clamp(g)) as u32;
        let bbyte = (256.0 * INTENSITY.clamp(b)) as u32;

        println!("{} {} {}", rbyte, gbyte, bbyte);
    }

    pub fn random() -> Self {
        Self {
            r: random_float(),
            g: random_float(),
            b: random_float(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            r: random_float_range(min, max),
            g: random_float_range(min, max),
            b: random_float_range(min, max),
        }
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

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
