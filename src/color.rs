use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    println!("{} {} {}\n", pixel_color.x(), pixel_color.y(), pixel_color.z());
}
