use crate::hittable;
use crate::vec3;
use crate::ray;
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r:&ray::Ray,  t_min:f64, t_max:f64, rec:&mut hittable::HitRecord) -> bool {
        let oc = &r.orig - &self.center;
        let a = r.dir.length_squared();
        let half_b = vec3::dot(&r.dir, &oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable rane
        let mut root = (- half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (- half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = &(&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        return true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::hittable::Hittable;
    use crate::material::Lambertian;
    #[test]
    fn test_hit() {
        let x = Sphere{center: vec3::point3(0.0, 0.0, -1.0), radius: 0.5, mat_ptr: Rc::new(Lambertian{albedo:vec3::color(0.7, 0.3, 0.3)})};
        let mut hit_rec = hittable::hit_record();
        let result = x.hit(&ray::Ray{orig: vec3::zero(), dir: vec3::vec3(0.0, 0.0, -1.0)}, 0.0, 2.0, &mut hit_rec);
        assert_eq!(result, true);
        //assert_eq!(hit_rec, hittable::HitRecord{p: vec3::point3(0.0, 0.0, -0.5), normal: vec3::vec3(0.0, 0.0, 1.0), t:0.5, front_face: true, });

        let result = x.hit(&ray::Ray{orig: vec3::zero(), dir: vec3::vec3(0.0, 0.0, -1.0)}, 1.0, 2.0, &mut hit_rec);
        assert_eq!(result, true);
        //assert_eq!(hit_rec, hittable::HitRecord{p: vec3::point3(0.0, 0.0, -1.5), normal: vec3::vec3(0.0, 0.0, 1.0), t:1.5, front_face: false, });
    }
}
