use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material::Material;
use super::point3::Point3;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Default)]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: Option<&'a dyn Material>,
}

impl<'a> Sphere<'a> {
    pub fn default() -> Sphere<'a> {
        Sphere {
            center: Point3::default(),
            radius: f64::default(),
            material: None,
        }
    }
}

impl<'a> Hittable<'a> for Sphere<'a> {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord<'a>) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.norm_squared();
        let h = Vec3::dot(&ray.direction, &oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // checks if either root satisfies the range requirement
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        // Set hit record data
        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = self.material;

        return true;
    }
}
