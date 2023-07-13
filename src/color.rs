mod vec3;

fn write_color(pixelColor: vec3::Color) {
    println!("{} {} {}\n", pixelColor.x(), pixelColor.y(), pixelColor.z());
}
