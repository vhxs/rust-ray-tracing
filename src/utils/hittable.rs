use super::interval::Interval;
use super::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

#[derive(Default)]
pub struct HittableList<'a, T: Hittable> {
    pub objects: Vec<&'a T>,
}

impl<'a, T: Hittable> HittableList<'a, T> {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &'a T) {
        self.objects.push(object);
    }
}

impl<'a, T: Hittable> Hittable for HittableList<'a, T> {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut some_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let interval = Interval {
                min: ray_t.min,
                max: closest_so_far,
            };
            if object.hit(ray, interval, &mut some_record) {
                hit_anything = true;
                closest_so_far = some_record.t;
                *hit_record = some_record.clone();
            }
        }

        return hit_anything;
    }
}
