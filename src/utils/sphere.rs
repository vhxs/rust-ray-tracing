use super::point3::Point3;
use super::ray::Ray;
use super::vec3::dot;

pub fn hit_sphere(center: &Point3, radius: &f64, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2. * dot(&ray.direction, &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discrminant = b * b - 4. * a * c;

    return discrminant >= 0.;
}
