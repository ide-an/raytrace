use crate::vec3;

// ostreamを受け取るのがわからんのでいったんなしで
#[allow(dead_code)]
pub fn write_color(color:&vec3::Color) {
    let ir:u8 = (color.x * 255.99) as u8;
    let ig:u8 = (color.y * 255.99) as u8;
    let ib:u8 = (color.z * 255.99) as u8;
    println!("{} {} {}", ir, ig, ib)
}
