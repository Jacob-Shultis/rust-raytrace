mod vec3;

fn write_color(pixelColor: vec3::Color) {
    eprintln("{} {} {}\n", pixelColor.x(), pixelColor.y(), pixelColor.z());
}
