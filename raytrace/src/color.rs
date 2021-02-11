use crate::vec3;
use crate::common::clamp;

// ostreamを受け取るのがわからんのでいったんなしで
#[allow(dead_code)]
//pub fn write_color(color:&vec3::Color) {
//    let ir:u8 = (color.x * 255.99) as u8;
//    let ig:u8 = (color.y * 255.99) as u8;
//    let ib:u8 = (color.z * 255.99) as u8;
//    println!("{} {} {}", ir, ig, ib)
//}
pub fn write_color(pixel_color:&vec3::Color, sample_per_pixel: i32) {
    let scale = 1.0 / sample_per_pixel as f64;
    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;
    println!("{} {} {}",
             (256.0 * clamp(r, 0.0, 0.999)) as u8,
             (256.0 * clamp(g, 0.0, 0.999)) as u8,
             (256.0 * clamp(b, 0.0, 0.999)) as u8
             )
}
