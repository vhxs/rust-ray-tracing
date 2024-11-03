use crate::utils::{
    color::write_color,
    point3::Point3,
    ray::{ray_color, Ray},
    vec3::Vec3,
};

pub fn run() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

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

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        }
        - viewport_u / 2.
        - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!(
        "P3\n{image_width} {image_height}\n255\n",
        image_width = image_width,
        image_height = image_height
    );

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
}
