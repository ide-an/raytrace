use std::ops;
use crate::common::{random_double, random_double_range};

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn vec3(x:f64, y:f64, z:f64) -> Vec3 {
    Vec3{x: x, y: y, z: z}
}

pub fn zero() -> Vec3 {
    vec3(0.0, 0.0, 0.0)
}
#[allow(dead_code)]
impl Vec3 {

    pub fn minus(&self) -> Self {
        vec3( - self.x,  - self.y,  - self.z )
    }

    pub fn at(&self, i:u8) -> f64{
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("invalid i"),
        }
    }

    pub fn at_assign(&mut self, i:u8, v:f64) -> &mut Self {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            2 => self.z = v,
            _ => panic!("invalid i"),
        }
        self
    }

    pub fn add_assign(&mut self, v:&Vec3) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }

    pub fn mul_assign(&mut self, f:f64) -> &mut Self {
        self.x *= f;
        self.y *= f;
        self.z *= f;
        self
    }

    pub fn div_assign(&mut self, f:f64) -> &mut Self {
        self.mul_assign(1.0/f)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn tostr(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }

}
#[allow(dead_code)]
impl<'a1,'a2> ops::Add<&'a2 Vec3> for &'a1 Vec3 {
    type Output = Vec3;
    fn add(self, b:&'a2 Vec3) -> Vec3  {
        vec3(self.x+b.x, self.y+b.y, self.z+b.z)
    }
}
#[allow(dead_code)]
impl<'a1,'a2> ops::Sub<&'a2 Vec3> for &'a1 Vec3 {
    type Output = Vec3;
    fn sub(self, b:&'a2 Vec3) -> Vec3  {
        vec3(self.x-b.x, self.y-b.y, self.z-b.z)
    }
}
#[allow(dead_code)]
pub fn mul(a:&Vec3, b:&Vec3) -> Vec3  {
    vec3(a.x*b.x, a.y*b.y, a.z*b.z)
}

#[allow(dead_code)]
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, f:f64) -> Vec3  {
        vec3(self.x*f, self.y*f, self.z*f)
    }
}

#[allow(dead_code)]
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, f:f64) -> Vec3  {
        self * (1.0/f)
    }
}

#[allow(dead_code)]
pub fn dot(a:&Vec3, b:&Vec3) -> f64  {
    a.x*b.x + a.y*b.y + a.z*b.z
}

#[allow(dead_code)]
pub fn cross(a:&Vec3, b:&Vec3) -> Vec3  {
    vec3(
        a.y*b.z - a.z*b.y,
        a.z*b.x - a.x*b.z,
        a.x*b.y - a.y*b.x
        )
}

#[allow(dead_code)]
pub fn unit_vector(a:&Vec3) -> Vec3 {
    a / a.length()
}

pub fn random() -> Vec3 {
    vec3(random_double(), random_double(), random_double())
}
pub fn random_range(low:f64, high:f64) -> Vec3 {
    vec3(random_double_range(low, high), random_double_range(low, high), random_double_range(low, high))
}
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

#[allow(dead_code)]
pub type Point3 = Vec3;
pub fn point3(x:f64, y:f64, z:f64) -> Point3 {
    Point3{x: x, y: y, z: z}
}
#[allow(dead_code)]
pub type Color = Vec3;
pub fn color(r:f64, g:f64, b:f64) -> Color {
    Color{x: r, y: g, z: b}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3() {
        let v = zero();
        eprint!("v: {:?}\n", v);
        let mut w = vec3(1.0, 2.0, 3.0);
        eprint!("w: {:?}\n", w);
        eprint!("w: {:?}\n", w.minus());
        eprint!("w: {:?}\n", w.minus().at(0));
        assert_eq!(w.minus().at(0), -1.0);
        w.at_assign(0, -10.0);
        assert_eq!(w.at(0), -10.0);
        let mut x = vec3(1.0, 2.0, 3.0);
        x.add_assign(&vec3(1.0, 2.0, 3.0));
        assert_eq!(x, vec3(2.0, 4.0, 6.0));
        let mut y = vec3(1.0, 2.0, 3.0);
        y.mul_assign(0.5);
        assert_eq!(y, vec3(0.5, 1.0, 1.5));
        assert_eq!(vec3(4.0, 3.0, 0.0).length(), 5.0);
        assert_eq!(vec3(4.0, 3.0, 0.0).length_squared(), 25.0);
    }

    #[test]
    fn test_vec3_static() {
        assert_eq!(&vec3(1.0, 2.0, 3.0) + &vec3(1.0, 2.0, 3.0), vec3(2.0, 4.0, 6.0));
        assert_eq!(dot(&vec3(1.0, 2.0, 3.0), &vec3(1.0, 2.0, 3.0)), 14.0);
        assert_eq!(cross(&vec3(1.0, 2.0, 3.0), &vec3(3.0, 2.0, 1.0)), vec3(-4.0, 8.0, -4.0));
    }
}
