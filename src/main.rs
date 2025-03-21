mod raytracing;

use raytracing::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    materials::{Dielectric, Lambertian, Metal},
    random_float, random_float_range,
    shapes::Sphere,
    vec3::{Point, Vec3},
};

// reading from this:
// https://raytracing.github.io/books/RayTracingInOneWeekend.html

fn main() {
    let mut world = HittableList { objects: vec![] };

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));

    let distance_point = Point::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point::new(
                a as f64 + (0.9 * random_float()),
                0.2,
                b as f64 + (0.9 * random_float()),
            );

            if (center - distance_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);

                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);

                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);

                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material3),
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point::new(13.0, 2.0, 3.0);
    camera.lookat = Point::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(world);
}
