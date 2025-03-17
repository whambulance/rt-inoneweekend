use crate::raytracing::{color::Color, vec3::Vec3};

use super::{
    hittable::{HitRecord, HittableList},
    interval::Interval,
};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn default() -> Self {
        Self {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn at(&self, f: f64) -> Vec3 {
        self.origin + (self.direction * f)
    }

    pub fn color(&self, world: &HittableList, depth: u32) -> Color {
        if depth == 0 {
            return Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        let mut hit_record: HitRecord = HitRecord::default();

        if world.hit(
            self,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
            &mut hit_record,
        ) {
            let mut scattered: Ray = Ray::default();

            if hit_record
                .material
                .scatter(self, &hit_record, &mut scattered)
            {
                let attenuation = hit_record.material.attenuation();
                return scattered.color(world, depth - 1) * attenuation;
            } else {
                return Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                };
            }
            // let direction = hit_record.normal + Vec3::random_unit_vector();
            // let new_ray = Ray {
            //     origin: hit_record.point,
            //     direction,
            // };

            // return new_ray.color(world, depth - 1) * 0.7;
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
}
