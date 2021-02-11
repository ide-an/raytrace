mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod common;
mod camera;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::common::*;

fn ray_color(r:&ray::Ray, world: &dyn hittable::Hittable, depth:i32) -> vec3::Color {
    if depth <= 0 {
        return vec3::color(0.0, 0.0, 0.0);
    }
    let mut hit_rec = hittable::hit_record();
    if world.hit(&r, 0.0, common::INFINITY, &mut hit_rec) {
        let target = hit_rec.p + hit_rec.normal + vec3::random_in_unit_sphere();
        return ray_color(&ray::Ray{orig:hit_rec.p, dir:target - hit_rec.p}, world, depth - 1) * 0.5;
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
    let sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = hittable_list::HittableList{ objects: Vec::new() };
    world.add(Box::new(sphere::Sphere{center: vec3::point3(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Box::new(sphere::Sphere{center: vec3::point3(0.0, -100.5, -1.0), radius: 100.0}));

    // Camera
    let camera = Camera::camera();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = vec3::color(0.0, 0.0, 0.0);
            for s in 0..sample_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            color::write_color(&pixel_color, sample_per_pixel);
        }
    }
    eprint!("\nDone.\n");
}
