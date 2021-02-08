fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        for i in (0..image_width) {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir:u8 = (r * 255.99) as u8;
            let ig:u8 = (g * 255.99) as u8;
            let ib:u8 = (b * 255.99) as u8;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
