#[derive(Debug,PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn vec3(x:f64, y:f64, z:f64) -> Self {
        Vec3{x: x, y: y, z: z}
    }
    
    pub fn zero() -> Self {
        Self::vec3(0.0, 0.0, 0.0)
    }

    pub fn minus(&self) -> Self {
        Self::vec3( - self.x,  - self.y,  - self.z )
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
    pub fn add(a:&Vec3, b:&Vec3) -> Self  {
        Self::vec3(a.x+b.x, a.y+b.y, a.z+b.z)
    }
    pub fn sub(a:&Vec3, b:&Vec3) -> Self  {
        Self::vec3(a.x-b.x, a.y-b.y, a.z-b.z)
    }

    pub fn mul(a:&Vec3, b:&Vec3) -> Self  {
        Self::vec3(a.x*b.x, a.y*b.y, a.z*b.z)
    }

    pub fn mul_scalar(a:&Vec3, f:f64) -> Self  {
        Self::vec3(a.x*f, a.y*f, a.z*f)
    }

    pub fn div_scalar(a:&Vec3, f:f64) -> Self  {
        Self::mul_scalar(a, 1.0/f)
    }

    pub fn dot(a:&Vec3, b:&Vec3) -> f64  {
        a.x*b.x + a.y*b.y + a.z*b.z
    }

    pub fn cross(a:&Vec3, b:&Vec3) -> Self  {
        Self::vec3(
            a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x
            )
    }

    pub fn unit_vector(a:&Vec3) -> Self {
        Self::div_scalar(a, a.length())
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3() {
        let v = Vec3::zero();
        eprint!("v: {:?}\n", v);
        let mut w = Vec3::vec3(1.0, 2.0, 3.0);
        eprint!("w: {:?}\n", w);
        eprint!("w: {:?}\n", w.minus());
        eprint!("w: {:?}\n", w.minus().at(0));
        assert_eq!(w.minus().at(0), -1.0);
        w.at_assign(0, -10.0);
        assert_eq!(w.at(0), -10.0);
        let mut x = Vec3::vec3(1.0, 2.0, 3.0);
        x.add_assign(&Vec3::vec3(1.0, 2.0, 3.0));
        assert_eq!(x, Vec3::vec3(2.0, 4.0, 6.0));
        let mut y = Vec3::vec3(1.0, 2.0, 3.0);
        y.mul_assign(0.5);
        assert_eq!(y, Vec3::vec3(0.5, 1.0, 1.5));
        assert_eq!(Vec3::vec3(4.0, 3.0, 0.0).length(), 5.0);
        assert_eq!(Vec3::vec3(4.0, 3.0, 0.0).length_squared(), 25.0);
    }

    #[test]
    fn test_vec3_static() {
        assert_eq!(Vec3::dot(&Vec3::vec3(1.0, 2.0, 3.0), &Vec3::vec3(1.0, 2.0, 3.0)), 14.0);
        assert_eq!(Vec3::cross(&Vec3::vec3(1.0, 2.0, 3.0), &Vec3::vec3(3.0, 2.0, 1.0)), Vec3::vec3(-4.0, 8.0, -4.0));
    }
}
