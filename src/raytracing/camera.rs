use crate::raytracing::color::Color;
use crate::raytracing::ray::Ray;
use crate::raytracing::vec3::{Point, Vec3};

use super::hittable::HittableList;
use super::random_float;

pub struct Camera {
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub max_depth: u32,

    pub image_width: u32,
    pub image_height: u32,

    pub camera_center: Point,

    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,

    pub pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        viewport_height: f64,
        focal_length: f64,
        camera_center: Point,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        // let image_height = Self::image_height(image_width, aspect_ratio);
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }

        // let viewport_width = Self::viewport_width(viewport_height, image_width, image_height);
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // let viewport_u = Self::viewport_u(viewport_width);
        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        //let viewport_v = Self::viewport_v(viewport_height);
        let viewport_v = Vec3 {
            x: 0.0,
            y: (viewport_height * -1.0),
            z: 0.0,
        };

        // let pixel_delta_u = Self::pixel_delta_u(viewport_u, image_width);
        let pixel_delta_u = viewport_u / image_width as f64;
        // let pixel_delta_v = Self::pixel_delta_v(viewport_v, image_height);
        let pixel_delta_v = viewport_v / image_height as f64;

        // let viewport_upper_left =
        //     Self::viewport_upper_left(camera_center, focal_length, viewport_u, viewport_v);
        let focal_vec = Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
        let viewport_upper_left =
            camera_center - focal_vec - (viewport_u / 2.0) - (viewport_v / 2.0);

        // let pixel00_loc = Self::pixel00_loc(viewport_upper_left, pixel_delta_u, pixel_delta_v);
        let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        eprintln!("aspect ratio: {}", aspect_ratio);
        eprintln!("image_height: {}", image_height);
        eprintln!("image_width: {}", image_width);

        eprintln!("viewport_height: {}", viewport_height);
        eprintln!("viewport_width: {}", viewport_width);
        eprintln!("viewport_u: {}", viewport_u);
        eprintln!("viewport_v: {}", viewport_v);
        eprintln!("pixel_delta_u: {}", pixel_delta_u);
        eprintln!("pixel_delta_v: {}", pixel_delta_v);
        eprintln!("upperleft: {}", viewport_upper_left);
        eprintln!("pixel00_loc: {}", pixel00_loc);

        Self {
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            image_width,
            image_height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn get_ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + x))
            + (self.pixel_delta_v * (offset.y + y));

        // let pixel_center = self.pixel00_loc + (self.pixel_delta_u * x) + (self.pixel_delta_v * y);
        let ray_direction = pixel_sample - self.camera_center;

        Ray {
            origin: self.camera_center,
            direction: ray_direction,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3 {
            x: random_float() - 0.5,
            y: random_float() - 0.5,
            z: 0.0,
        }
    }

    pub fn render(&self, world: HittableList) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {}   ", (self.image_height - y));
            for x in 0..self.image_width {
                let mut pixel_color = Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                };
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray_for_pixel(x as f64, y as f64);
                    pixel_color += ray.color(&world, self.max_depth);
                }

                pixel_color *= self.pixel_samples_scale;
                pixel_color.write();
            }
        }

        eprint!("\rDone.                                 \n");
    }
}
