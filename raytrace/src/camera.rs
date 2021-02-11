use crate::vec3::{Point3, Vec3, vec3, point3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn camera() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_heiht = 2.0;
        let viewport_width = aspect_ratio * viewport_heiht;
        let focal_length = 1.0;

        let origin = point3(0.0, 0.0, 0.0);
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, viewport_heiht, 0.0);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - vec3(0.0, 0.0, focal_length);
        return Camera{
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal:horizontal,
            vertical:vertical
        }
    }
    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        return Ray{
            orig: self.origin,
            dir: self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        }
    }
}
