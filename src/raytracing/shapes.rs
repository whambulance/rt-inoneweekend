use crate::raytracing::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{dot, Vec3},
};

use super::{interval::Interval, materials::Material};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = dot(ray.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.clone();

        true
    }
}
