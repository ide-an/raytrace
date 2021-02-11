use crate::vec3;
use crate::ray;
use crate::material::Material;
use crate::material::Null;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}
pub fn hit_record() -> HitRecord {
    HitRecord{p:vec3::zero(), normal: vec3::zero(), t:0.0, front_face: false, mat_ptr: Rc::new(Null{}) }
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { outward_normal * -1.0}; 
    }
}

pub trait Hittable {
    fn hit(&self, r:&ray::Ray,  t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool;
}
