use super::hittable::{HitRecord, Hittable};
use super::point3::Point3;
use super::ray::Ray;
use super::vec3::dot;

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: &f64, ray_tmax: &f64, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = dot(&ray.direction, &oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // checks if either root satisfies the range requirement
        let mut root = (h - sqrtd) / a;
        if root <= *ray_tmin || *ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= *ray_tmin || *ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        return true;
    }
}
