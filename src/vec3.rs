use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: (u.y * v.z) - (u.z * v.y),
        y: (u.z * v.x) - (u.x * v.z),
        z: (u.x * v.y) - (u.y * v.z),
    }
}
