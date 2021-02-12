use crate::vec3::{Point3, Vec3, vec3, point3,unit_vector,cross,random_in_unit_disk};
use crate::ray::Ray;
use crate::common::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}


impl Camera {
    pub fn camera(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64
        ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_heiht = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_heiht;

        let w = unit_vector(&(&lookfrom - &lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = &u * (viewport_width * focus_dist);
        let vertical = &v * (viewport_heiht * focus_dist);
        let lower_left_corner = &(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &(&w * focus_dist);
        let lens_radius = aperture / 2.0;
        return Camera{
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal:horizontal,
            vertical:vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }
    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        let rd = &random_in_unit_disk() * self.lens_radius;
        let offset = &(&self.u * rd.x) + &(&self.v * rd.y);
        return Ray{
            orig: &self.origin + &offset,
            dir: &(&(&(&self.lower_left_corner + &(&self.horizontal * s)) + &(&self.vertical * t)) - &self.origin) - &offset,
        }
    }
}
