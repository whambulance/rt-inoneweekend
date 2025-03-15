mod raytracing;

use raytracing::{camera::Camera, hittable::HittableList, shapes::Sphere, vec3::Point};

// reading from this:
// https://raytracing.github.io/books/RayTracingInOneWeekend.html

fn main() {
    let camera_center = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let camera = Camera::new(16.0 / 9.0, 400, 2.0, 1.0, camera_center);

    let mut world = HittableList { objects: vec![] };

    let sphere1 = Sphere::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
    );
    world.add(Box::new(sphere1));

    let sphere2 = Sphere::new(
        Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        100.0,
    );
    world.add(Box::new(sphere2));

    println!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    for y in 0..camera.image_height {
        eprint!("\rScanlines remaining: {}", (camera.image_height - y));
        for x in 0..camera.image_width {
            let ray = camera.get_ray_for_pixel(x as f64, y as f64);
            let color = ray.color(&world);

            color.write();
        }
    }

    eprint!("\rDone.                                 \n");
}
