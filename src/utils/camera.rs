use super::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn initialize(aspect_ratio: f64, image_width: u32) -> Camera {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        // Viewport
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.,
            z: 0.,
        };
        let viewport_v = Vec3 {
            x: 0.,
            y: -viewport_height,
            z: 0.,
        };

        // Pixel deltas
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left = center
            - Vec3 {
                x: 0.,
                y: 0.,
                z: focal_length,
            }
            - viewport_u / 2.
            - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        let mut hit_record = HitRecord::default();
        let interval = Interval {
            min: 0.,
            max: f64::INFINITY,
        };
        if world.hit(ray, interval, &mut hit_record) {
            return (hit_record.normal
                + Color {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                })
                * 0.5;
        }

        let unit_direction = Vec3::unit_vector(&ray.direction);
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

    pub fn render(&self, world: &dyn Hittable) {
        print!(
            "P3\n{image_width} {image_height}\n255\n",
            image_width = self.image_width,
            image_height = self.image_height
        );

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let r = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };

                let pixel_color = Camera::ray_color(&r, world);
                write_color(&pixel_color);
            }
        }
    }
}
