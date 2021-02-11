use rand::{thread_rng, Rng};
pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    thread_rng().gen_range(0.0, 1.0)
}
pub fn random_double_range(low:f64, high:f64) -> f64 {
    thread_rng().gen_range(low, high)
}

pub fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_degree() {
        assert_eq!(degrees_to_radians(90.0), PI/2.0);
    }
}
