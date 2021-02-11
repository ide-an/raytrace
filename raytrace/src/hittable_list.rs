use crate::ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::hittable;

pub struct HittableList {
   pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:&ray::Ray,  t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool {
        let mut tmp_rec = hittable::hit_record();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone()
            }
        }
        return hit_anything
    }
}
#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;
    use crate::vec3;
    use super::*;
    use std::rc::Rc;
    use crate::material::Lambertian;
    #[test]
    fn test_hit() {
        let mut hitlist = HittableList{objects: Vec::new() };
        let x = Box::new(Sphere{center: vec3::point3(0.0, 0.0, -1.0), radius: 0.5, mat_ptr: Rc::new(Lambertian{albedo:vec3::color(0.7, 0.3, 0.3)})});
        hitlist.add(x);
        let mut hit_rec = hittable::hit_record();
        let result = hitlist.hit(&ray::Ray{orig: vec3::zero(), dir: vec3::vec3(0.0, 0.0, -1.0)}, 0.0, 2.0, &mut hit_rec);
        assert_eq!(result, true);
        //assert_eq!(hit_rec, hittable::HitRecord{p: vec3::point3(0.0, 0.0, -0.5), normal: vec3::vec3(0.0, 0.0, 1.0), t:0.5, front_face: true, });

        let result = hitlist.hit(&ray::Ray{orig: vec3::zero(), dir: vec3::vec3(0.0, 0.0, -1.0)}, 1.0, 2.0, &mut hit_rec);
        assert_eq!(result, true);
        //assert_eq!(hit_rec, hittable::HitRecord{p: vec3::point3(0.0, 0.0, -1.5), normal: vec3::vec3(0.0, 0.0, 1.0), t:1.5, front_face: false, });
    }
}
