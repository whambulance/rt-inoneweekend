use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use super::{random_float, random_float_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8_f64;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if dot(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    pub fn reflect(&self, other: Vec3) -> Vec3 {
        *self - other * dot(*self, other) * 2.0
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta: f64 = dot(*self * -1.0, normal).min(1.0);
        let r_out_perp: Vec3 = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel: Vec3 = normal * (1.0 - r_out_perp.length_squared()).abs().sqrt() * -1.0;

        r_out_perp + r_out_parallel
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_float(),
            y: random_float(),
            z: random_float(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_float_range(min, max),
            y: random_float_range(min, max),
            z: random_float_range(min, max),
        }
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
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
        // (self.x * other.x) + (self.y * other.y)
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
        z: (u.x * v.y) - (u.y * v.x),
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_float_range(-1.0, 1.0),
            random_float_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
