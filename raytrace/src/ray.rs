use crate::vec3;

pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn at(&self, t:f64) -> vec3::Point3 {
        self.orig + (self.dir * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray() {
        let ray = Ray{orig: vec3::Vec3::vec3(1.0, 2.0, 3.0), dir: vec3::Vec3::vec3(1.0, 2.0, 3.0)};
        assert_eq!(ray.at(0.5), vec3::Vec3::vec3(1.5,3.0,4.5))
    }
}
