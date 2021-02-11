use crate::vec3;
use crate::common::clamp;

// ostreamを受け取るのがわからんのでいったんなしで
#[allow(dead_code)]
pub fn write_color(pixel_color:&vec3::Color, sample_per_pixel: i32) {
    let scale = 1.0 / sample_per_pixel as f64;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    println!("{} {} {}",
             (256.0 * clamp(r, 0.0, 0.999)) as u8,
             (256.0 * clamp(g, 0.0, 0.999)) as u8,
             (256.0 * clamp(b, 0.0, 0.999)) as u8
             )
}
