mod vec3;

fn main() {
    let v = vec3::Vec3::zero();
    eprint!("v: {:?}\n", v);
    let w = vec3::Vec3::vec3(1.0, 2.0, 3.0);
    eprint!("w: {:?}\n", w);
    eprint!("w: {:?}\n", w.minus());
    eprint!("w: {:?}\n", w.minus().at(0));

    let c:vec3::Color = vec3::Vec3::vec3(1.0, 2.0, 3.0);
    eprint!("w: {:?}\n", c);

    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir:u8 = (r * 255.99) as u8;
            let ig:u8 = (g * 255.99) as u8;
            let ib:u8 = (b * 255.99) as u8;
            println!("{} {} {}", ir, ig, ib)
        }
    }
        eprint!("\nDone.\n");
}
