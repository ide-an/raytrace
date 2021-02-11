use crate::vec3;
use crate::ray;

struct HitRecord {
    p: vec3::Point3,
    normal: vec3::Vec3,
    t: f64,
}

trait Hittable {
    fn hit(&self, r:&ray::Ray,  t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}
