use super::vec3::Vec3;

pub type Point3 = Vec3;

impl Point3 {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }
}
