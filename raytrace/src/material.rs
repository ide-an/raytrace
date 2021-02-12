use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Color;
use crate::vec3;
use crate::common::random_double;

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

pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in:&Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = vec3::color(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = vec3::unit_vector(&r_in.dir);
        let theta_dot = vec3::dot(&(&unit_direction * -1.0), &rec.normal);
        let cos_theta = if theta_dot < 1.0 { theta_dot } else { 1.0 };
        let sin_theta = (1.0f64 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }
        *scattered = Ray{ orig: rec.p, dir: direction };
        return true;
    }

}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
