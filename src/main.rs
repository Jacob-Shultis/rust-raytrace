use std::io::Write;
mod vec3;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;
mod ray;
use ray::Ray;
mod color;

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;
    let a: f32 = r.direction().length_squared();
    let half_b: f32 = oc.dot(r.direction());
    let c: f32 = oc.length_squared() - radius*radius;
    let discriminant: f32 = half_b*half_b - a*c;
    
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f32::sqrt(discriminant)) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t: f32 = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

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
