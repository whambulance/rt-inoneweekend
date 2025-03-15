use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,

    pub image_width: u32,
    pub image_height: u32,

    pub viewport_height: f64,
    pub viewport_width: f64,

    pub focal_length: f64,
    pub camera_center: Point,

    pub viewport_u: Vec3,
    pub viewport_v: Vec3,

    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,

    pub viewport_upper_left: Vec3,
    pub pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        viewport_height: f64,
        focal_length: f64,
        camera_center: Point,
    ) -> Self {
        let image_height = Self::image_height(image_width, aspect_ratio);
        let viewport_width = Self::viewport_width(viewport_height, image_width, image_height);

        let viewport_u = Self::viewport_u(viewport_width);
        let viewport_v = Self::viewport_v(viewport_height);

        let pixel_delta_u = Self::pixel_delta_u(viewport_u, image_width);
        let pixel_delta_v = Self::pixel_delta_v(viewport_v, image_height);

        let viewport_upper_left =
            Self::viewport_upper_left(camera_center, focal_length, viewport_u, viewport_v);
        let pixel00_loc = Self::pixel00_loc(viewport_upper_left, pixel_delta_u, pixel_delta_v);

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
            aspect_ratio,
            image_width,
            image_height,
            viewport_height,
            viewport_width,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
        }
    }

    fn image_height(image_width: u32, aspect_ratio: f64) -> u32 {
        let mut image_height = image_width as f64 / aspect_ratio;
        if image_height < 1.0 {
            image_height = 1.0;
        }
        image_height as u32
    }

    fn viewport_width(viewport_height: f64, image_width: u32, image_height: u32) -> f64 {
        viewport_height * (image_width as f64 / image_height as f64)
    }

    fn viewport_u(viewport_width: f64) -> Vec3 {
        Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        }
    }

    fn viewport_v(viewport_height: f64) -> Vec3 {
        Vec3 {
            x: 0.0,
            y: (viewport_height * -1.0),
            z: 0.0,
        }
    }

    fn viewport_upper_left(
        camera_center: Point,
        focal_length: f64,
        viewport_u: Vec3,
        viewport_v: Vec3,
    ) -> Vec3 {
        let focal_vec = Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
        camera_center - focal_vec - (viewport_u / 2.0) - (viewport_v / 2.0)
    }

    fn pixel_delta_u(viewport_u: Vec3, image_width: u32) -> Vec3 {
        viewport_u / image_width as f64
    }

    fn pixel_delta_v(viewport_v: Vec3, image_height: u32) -> Vec3 {
        viewport_v / image_height as f64
    }

    fn pixel00_loc(viewport_upper_left: Vec3, pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Vec3 {
        viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5)
    }

    pub fn get_ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * x) + (self.pixel_delta_v * y);
        let ray_direction = pixel_center - self.camera_center;

        Ray {
            origin: self.camera_center,
            direction: ray_direction,
        }
    }
}
