use crate::raytracing::{
    color::Color,
    vec3::{dot, Point, Vec3},
};

use super::hittable::{HitRecord, HittableList};
use super::INFINITY;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, f: f64) -> Vec3 {
        self.origin + (self.direction * f)
    }

    pub fn color(&self, world: &HittableList) -> Color {
        // let sphere: Sphere;

        // let sphere_center = Vec3 {
        //     x: 0.0,
        //     y: 0.0,
        //     z: -1.0,
        // };

        // let t = self.hit_sphere(sphere_center, 0.5);

        // if t > 0.0 {
        //     let sphere_n = self.at(t) - sphere_center;
        //     let n = sphere_n.unit_vector();
        //     let color = Color {
        //         r: n.x + 1.0,
        //         g: n.y + 1.0,
        //         b: n.z + 1.0,
        //     };
        //     return color * 0.5;
        // }

        let mut hit_record: HitRecord = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut hit_record) {
            let vector = (Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } + hit_record.normal)
                * 0.5;

            return Color {
                r: vector.x,
                g: vector.y,
                b: vector.z,
            };
        }

        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);

        let full = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let shaded = Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };

        let combined = (full * (1.0 - a)) + (shaded * a);

        Color {
            r: combined.x,
            g: combined.y,
            b: combined.z,
        }
    }

    pub fn hit_sphere(self, center: Point, radius: f64) -> f64 {
        let oc = center - self.origin;
        let a = self.direction.length_squared();
        let h = dot(self.direction, oc);
        let c = oc.length_squared() - radius * radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (h - discriminant.sqrt()) / a
        }
    }
}
