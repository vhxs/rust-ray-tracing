use core::f64;

use super::point3::Point3;
use super::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + (self.direction * t);
    }
}
