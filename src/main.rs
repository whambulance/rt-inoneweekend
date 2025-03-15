use camera::Camera;
use vec3::Point;

mod camera;
mod color;
mod hittable;
mod point;
mod ray;
mod shapes;
mod vec3;

// reading from this:
// https://raytracing.github.io/books/RayTracingInOneWeekend.html

fn main() {
    let camera_center = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let camera = Camera::new(16.0 / 9.0, 400, 2.0, 1.0, camera_center);

    println!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    for y in 0..camera.image_height {
        eprint!("\rScanlines remaining: {}", (camera.image_height - y));
        for x in 0..camera.image_width {
            let ray = camera.get_ray_for_pixel(x as f64, y as f64);
            let color = ray.color();

            color.write();
        }
    }

    eprint!("\rDone.                                 \n");
}
