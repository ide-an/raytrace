pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_degree() {
        assert_eq!(degrees_to_radians(90.0), PI/2.0);
    }
}
