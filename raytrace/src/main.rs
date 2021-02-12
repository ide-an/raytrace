mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod common;
mod camera;
mod material;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::common::*;
use std::rc::Rc;

fn ray_color(r:&ray::Ray, world: &dyn hittable::Hittable, depth:i32) -> vec3::Color {
    if depth <= 0 {
        //eprint!("depth: {}\n", depth);
        return vec3::color(0.0, 0.0, 0.0);
    }
    let mut hit_rec = hittable::hit_record();
    if world.hit(&r, 0.001, common::INFINITY, &mut hit_rec) {
        let mut scattered = ray::Ray{ orig: vec3::zero(), dir: vec3::zero()};
        let mut attenuation = vec3::color(0.0, 0.0, 0.0);
        if hit_rec.mat_ptr.scatter(&r, &hit_rec, &mut attenuation, &mut scattered) {
            return vec3::mul(&ray_color(&scattered, world, depth - 1), &attenuation)
        }
        return vec3::color(0.0, 0.0, 0.0);
        //let target = &(&hit_rec.p + &hit_rec.normal) + &vec3::random_unit_vector();
        //return &ray_color(&ray::Ray{orig:hit_rec.p, dir:&target - &hit_rec.p}, world, depth - 1) * 0.5;
    }
        //eprint!("depth: {}\n", depth);
    let unit_direction = vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    &(&vec3::color(1.0, 1.0, 1.0) * (1.0 - t)) + &(&vec3::color(0.5, 0.7, 1.0) * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    //let image_width = 1600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = hittable_list::HittableList{ objects: Vec::new() };
    let material_ground = Rc::new(material::Lambertian{albedo: vec3::color(0.8, 0.8, 0.0)});
    //let material_center = Rc::new(material::Dielectric{ir: 1.5});
    let material_center = Rc::new(material::Lambertian{albedo: vec3::color(0.1, 0.2, 0.5)});
    let material_left = Rc::new(material::Dielectric{ir: 1.5});
    //let material_center = Rc::new(material::Lambertian{albedo: vec3::color(0.7, 0.3, 0.3)});
    //let material_left = Rc::new(material::Metal{albedo: vec3::color(0.8, 0.8, 0.8), fuzz: 0.3});
    let material_right = Rc::new(material::Metal{albedo: vec3::color(0.8, 0.6, 0.2), fuzz: 0.0});

    world.add(Box::new(sphere::Sphere{center: vec3::point3( 0.0,-100.5, -1.0), radius: 100.0, mat_ptr: material_ground}));
    world.add(Box::new(sphere::Sphere{center: vec3::point3( 0.0,   0.0, -1.0), radius:   0.5, mat_ptr: material_center}));
    world.add(Box::new(sphere::Sphere{center: vec3::point3(-1.0,   0.0, -1.0), radius:   0.5, mat_ptr: material_left.clone()}));
    world.add(Box::new(sphere::Sphere{center: vec3::point3(-1.0,   0.0, -1.0), radius:   -0.4, mat_ptr: material_left.clone()}));
    world.add(Box::new(sphere::Sphere{center: vec3::point3( 1.0,   0.0, -1.0), radius:   0.5, mat_ptr: material_right}));

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
                pixel_color = &pixel_color + &ray_color(&r, &world, max_depth);
            }
            color::write_color(&pixel_color, sample_per_pixel);
        }
    }
    eprint!("\nDone.\n");
}
