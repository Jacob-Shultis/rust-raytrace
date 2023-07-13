use std::io::Write;
mod vec3;
mod color;

fn main() {
    const IMAGE_WIDTH: i32 = 255;
    const IMAGE_HEIGHT: i32 = 255;

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for i in (0..IMAGE_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {i}");
        
        std::io::stderr().flush().expect("Unable to flush stderr");

        for j in 0..IMAGE_WIDTH {
            let pixel_color: vec3::color = vec3::color::new((j as f32) / (image_width - 1), (i as f32) / (image_height - 1), 0.25);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\nDone.\n");

    let v1: vec3::Point3 = vec3::Point3::new(2.0, 2.0, 2.0);
    let v2: vec3::Point3 = vec3::Point3::new(3.0, 3.0, 3.0);
    let v3 = 5.0 * v1;
    eprintln!("{}", v3);
}
