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
use crate::vec3::point3;

fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList{ objects: Vec::new() };

    let material_ground = Rc::new(material::Lambertian{albedo: vec3::color(0.5, 0.5, 0.5)});
    world.add(Box::new(sphere::Sphere{center: point3(0.0, -1000.0, 0.0), radius: 1000.0, mat_ptr: material_ground}));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = point3(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());
            if (&center - &point3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::random();
                    let sphere_material = Rc::new(material::Lambertian{albedo: albedo});
                    world.add(Box::new(sphere::Sphere{center: center, radius: 0.2, mat_ptr: sphere_material}));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal{albedo: albedo, fuzz: fuzz});
                    world.add(Box::new(sphere::Sphere{center: center, radius: 0.2, mat_ptr: sphere_material}));
                } else {
                    // glass
                    let sphere_material = Rc::new(material::Dielectric{ir: 1.5});
                    world.add(Box::new(sphere::Sphere{center: center, radius: 0.2, mat_ptr: sphere_material}));
                }
            }
        }
    }

    let material_1 = Rc::new(material::Dielectric{ir: 1.5});
    world.add(Box::new(sphere::Sphere{center: vec3::point3( 0.0,   1.0, 0.0), radius:   1.0, mat_ptr: material_1}));

    let material_2 = Rc::new(material::Lambertian{albedo: vec3::color(0.4, 0.2, 0.1)});
    world.add(Box::new(sphere::Sphere{center: vec3::point3(-4.0,   1.0, 0.0), radius:   1.0, mat_ptr: material_2}));

    let material_3 = Rc::new(material::Metal{albedo: vec3::color(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add(Box::new(sphere::Sphere{center: vec3::point3( 4.0,   1.0, 0.0), radius:   1.0, mat_ptr: material_3}));

    return world;
}

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let sample_per_pixel = 500;
    let max_depth = 50;

    // World
    let mut world = random_scene();

    // Camera
    let lookfrom = point3(13.0,2.0,3.0);
    let lookat = point3(0.0,0.0,0.0);
    let vup = vec3::vec3(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    //let dist_to_focus = (&lookfrom - &lookat).length();
    let aperture = 0.1;

    let camera = Camera::camera(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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
