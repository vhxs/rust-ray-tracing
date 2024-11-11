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

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.;
        let random_f64 = Vec3::random_coordinate(0., 1.);
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_f64 {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_record.normal, ri)
        };

        *scattered = Ray::new(hit_record.p, direction);
        return true;
    }
}

impl Dielectric {
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0 * r0;

        return r0 + (1. - r0) * f64::powf(1. - cosine, 5.);
    }
}
