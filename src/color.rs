use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}\n", (255.99 * pixel_color.x()) as i32, (255.99 * pixel_color.y()) as i32, (255.99 * pixel_color.z()) as i32);
}
