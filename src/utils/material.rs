use super::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray {
            origin: hit_record.p,
            direction: scatter_direction,
        };
        *attenuation = self.albedo.clone();

        return true;
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz > 1. { 1. } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(&ray_in.direction, &hit_record.normal);
        reflected = Vec3::unit_vector(&reflected) + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray {
            origin: hit_record.p,
            direction: reflected,
        };
        *attenuation = self.albedo.clone();

        return Vec3::dot(&scattered.direction, &hit_record.normal) > 0.;
    }
}

// bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
// const override {
//     attenuation = color(1.0, 1.0, 1.0);
//     double ri = rec.front_face ? (1.0/refraction_index) : refraction_index;

//     vec3 unit_direction = unit_vector(r_in.direction());
//     vec3 refracted = refract(unit_direction, rec.normal, ri);

//     scattered = ray(rec.p, refracted);
//     return true;
// }

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let ri = if hit_record.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(&ray_in.direction);
        let refracted = Vec3::refract(&unit_direction, &hit_record.normal, ri);

        *scattered = Ray::new(hit_record.p, refracted);
        return true;
    }
}
