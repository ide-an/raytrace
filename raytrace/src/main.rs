mod vec3;
mod color;
mod ray;

fn hit_sphere(center:&vec3::Point3, radius:f64, r:&ray::Ray) -> bool {
    let oc = r.orig - *center;
    let a = vec3::dot(&r.dir, &r.dir);
    let b = vec3::dot(&r.dir, &oc) * 2.0;
    let c = vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0
}
fn ray_color(r:&ray::Ray) -> vec3::Color {
    if hit_sphere(&vec3::vec3(0.0, 0.0, -1.0), 0.5, &r) {
        return vec3::color(1.0, 0.0, 0.0)
    }
    let unit_direction = vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (vec3::color(1.0, 1.0, 1.0) * (1.0 - t)) + (vec3::color(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_heiht = 2.0;
    let viewport_width = aspect_ratio * viewport_heiht;
    let focal_length = 1.0;

    let origin = vec3::point3(0.0, 0.0, 0.0);
    let horizontal = vec3::vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3::vec3(0.0, viewport_heiht, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - vec3::vec3(0.0, 0.0, focal_length);
    eprint!("{:?}\n", lower_left_corner);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = ray::Ray{
                orig: origin,
                dir: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };
            let pixel_color = ray_color(&r);
            color::write_color(&pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
