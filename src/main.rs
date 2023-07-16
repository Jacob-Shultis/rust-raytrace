use std::io::Write;
mod vec3;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;
mod ray;
use ray::Ray;
mod color;

fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

    // Camera

    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for i in (0..IMAGE_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {i}");
        
        std::io::stderr().flush().expect("Unable to flush stderr");

        for j in 0..IMAGE_WIDTH {
            let u: f32 = j as f32 / (IMAGE_WIDTH - 1) as f32;
            let v: f32 = i as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color: Color = ray_color(r);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");
}
