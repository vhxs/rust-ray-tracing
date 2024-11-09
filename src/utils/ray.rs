use core::f64;

use super::color::Color;
use super::hittable::HitRecord;
use super::hittable::Hittable;
use super::point3::Point3;
use super::vec3::unit_vector;
use super::vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + (self.direction * t);
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, &0., &f64::INFINITY, &mut hit_record) {
        return (hit_record.normal
            + Color {
                x: 1.,
                y: 1.,
                z: 1.,
            })
            * 0.5;
    }

    let unit_direction = unit_vector(&ray.direction);
    let a = unit_direction.y * 0.5 + 1.;
    return Color {
        x: 1.,
        y: 1.,
        z: 1.,
    } * (1. - a)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * a;
}
