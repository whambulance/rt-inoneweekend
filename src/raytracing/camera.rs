use crate::raytracing::color::Color;
use crate::raytracing::degrees_to_radians;
use crate::raytracing::ray::Ray;
use crate::raytracing::vec3::{cross, Point, Vec3};

use super::hittable::HittableList;
use super::random_float;
use super::vec3::random_in_unit_disk;

pub struct Camera {
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub image_width: u32,

    pub vfov: f64,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pixel_samples_scale: f64,
    image_height: u32,
    camera_center: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let samples_per_pixel = 100;
        let max_depth = 50;

        let vfov = 90.0;
        let lookfrom = Point::new(0.0, 0.0, 0.0);
        let lookat = Point::new(0.0, 0.0, 0.0);
        let vup = Point::new(0.0, 0.0, 0.0);

        let pixel_samples_scale = 0.0;
        let image_height = 0;
        let pixel_delta_u = Vec3::new(0.0, 0.0, 0.0);
        let pixel_delta_v = Vec3::new(0.0, 0.0, 0.0);
        let pixel00_loc = Vec3::new(0.0, 0.0, 0.0);
        let u = Vec3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 0.0, 0.0);
        let w = Vec3::new(0.0, 0.0, 0.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.0;
        let defocus_disk_u = Vec3::new(0.0, 0.0, 0.0);
        let defocus_disk_v = Vec3::new(0.0, 0.0, 0.0);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            image_height,
            pixel_samples_scale,
            camera_center: lookfrom,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).round() as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.camera_center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        eprintln!("lookfrom: {}", self.lookfrom);
        eprintln!("lookat: {}", self.lookat);
        eprintln!("vup: {}", self.vup);
        eprintln!("vfov: {}", self.vfov);
        eprintln!("theta: {theta}");
        eprintln!("h: {h}");

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = cross(self.vup, self.w).unit_vector();
        self.v = cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * viewport_height * -1.0;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.camera_center
            - (self.w * self.focus_dist)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);

        self.pixel00_loc = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        eprintln!("aspect ratio: {}", self.aspect_ratio);
        eprintln!("image_height: {}", self.image_height);
        eprintln!("image_width: {}", self.image_width);

        eprintln!("viewport_height: {}", viewport_height);
        eprintln!("viewport_width: {}", viewport_width);
        eprintln!("viewport_u: {}", viewport_u);
        eprintln!("viewport_v: {}", viewport_v);
        eprintln!("pixel_delta_u: {}", self.pixel_delta_u);
        eprintln!("pixel_delta_v: {}", self.pixel_delta_v);
        eprintln!("upperleft: {}", viewport_upper_left);
        eprintln!("pixel00_loc: {}", self.pixel00_loc);

        eprintln!("camera center: {}", self.camera_center);

        eprintln!("w: {}", self.w);
        eprintln!("u: {}", self.u);
        eprintln!("v: {}", self.v);

        eprintln!("defocus_angle: {}", self.defocus_angle);
        eprintln!("focus_dist: {}", self.focus_dist);
        eprintln!("defocus_disk_u: {}", self.defocus_disk_u);
        eprintln!("defocus_disk_v: {}", self.defocus_disk_v);
    }

    pub fn get_ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + x))
            + (self.pixel_delta_v * (offset.y + y));

        // let pixel_center = self.pixel00_loc + (self.pixel_delta_u * x) + (self.pixel_delta_v * y);
        let ray_origin: Point = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray {
            origin: ray_origin,
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

    fn defocus_disk_sample(&self) -> Point {
        let p = random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    pub fn render(&mut self, world: HittableList) {
        self.initialize();

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
