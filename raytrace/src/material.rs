use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Color;
use crate::vec3;

pub trait Material {
    fn scatter(&self, r_in:&Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = &rec.normal + &vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray{ orig: rec.p, dir: scatter_direction };
        *attenuation = self.albedo;
        return true
    }
}

pub struct Null { }

impl Material for Null {
    fn scatter(&self, r_in:&Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return false
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in:&Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(&vec3::unit_vector(&r_in.dir), &rec.normal);
        *scattered = Ray{ orig: rec.p, dir: &reflected + &(&vec3::random_in_unit_sphere() * self.fuzz)};
        *attenuation = self.albedo;
        return vec3::dot(&scattered.dir, &rec.normal) > 0.0;
    }
}
