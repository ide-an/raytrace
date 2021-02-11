use crate::vec3;
use crate::ray;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
}
pub fn hit_record() -> HitRecord {
    HitRecord{p:vec3::zero(), normal: vec3::zero(), t:0.0}
}

pub trait Hittable {
    fn hit(&self, r:&ray::Ray,  t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}
