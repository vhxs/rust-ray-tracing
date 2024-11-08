use super::color::Color;
use super::point3::Point3;
use super::sphere::hit_sphere;
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

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(
        &Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        &0.5,
        &ray,
    );

    if t > 0. {
        let vector = ray.at(t)
            - Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            };
        let normal = unit_vector(&vector);
        return Color {
            x: normal.x + 1.,
            y: normal.y + 1.,
            z: normal.z + 1.,
        } * 0.5;
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
