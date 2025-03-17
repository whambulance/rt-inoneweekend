mod raytracing;

use raytracing::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    materials::{Lambertian, Metal},
    shapes::Sphere,
    vec3::Point,
};

// reading from this:
// https://raytracing.github.io/books/RayTracingInOneWeekend.html

fn main() {
    let mut world = HittableList { objects: vec![] };

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Box::new(material_center),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));

    let camera_center = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        viewport_height,
        focal_length,
        camera_center,
        samples_per_pixel,
        max_depth,
    );

    camera.render(world);
}
