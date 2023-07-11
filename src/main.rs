use std::io::Write;

fn main() {
    const IMAGE_WIDTH: i32 = 255;
    const IMAGE_HEIGHT: i32 = 255;

    //std::cout << "P3\n" << IMAGE_WIDTH << ' ' << IMAGE_HEIGHT << '\n255\n';

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for i in (0..IMAGE_HEIGHT - 1).rev() {
        eprint!("\rScanlines remaining: {i}");
        
        std::io::stderr().flush().expect("Unable to flush stderr");

        for j in 0..IMAGE_WIDTH {
            let r: f32 = (j as f32) / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = (i as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}\n");
        }
    }

    eprintln!("\nDone.\n");
}
