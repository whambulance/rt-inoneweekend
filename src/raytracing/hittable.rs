use crate::raytracing::{
    ray::Ray,
    vec3::{dot, Point, Vec3},
};

use super::{
    color::Color,
    interval::Interval,
    materials::{Lambertian, Material},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            point: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material: Box::new(Lambertian {
                albedo: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                },
            }),
            front_face: false,
            t: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal
        } else {
            self.normal = outward_normal * -1.0
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let hit = object.hit(
                ray,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            );
            if hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
                rec.point = temp_rec.point;
                rec.t = temp_rec.t;
                rec.material = temp_rec.material.clone();
            }
        }

        hit_anything
    }
}
