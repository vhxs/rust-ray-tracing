use super::point3::Point3;
use super::ray::Ray;
use super::vec3::dot;

pub fn hit_sphere(center: &Point3, radius: &f64, ray: &Ray) -> f64 {
    let oc = *center - ray.origin;
    let a = dot(&ray.direction, &ray.direction);
    let b = -2. * dot(&ray.direction, &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discrminant = b * b - 4. * a * c;

    if discrminant < 0. {
        return -1.;
    } else {
        return (-b - discrminant.sqrt()) / (2. * a);
    }
}
