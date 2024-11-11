use crate::utils::interval::Interval;

use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn color(x: f64, y: f64, z: f64) -> Self {
        Color { x, y, z }
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    return 0.;
}

pub fn write_color(pixel_color: &Color) {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let intensity = Interval::bounded_interval(0., 0.999);
    let rbyte = (256. * intensity.clamp(r)) as u32;
    let gbyte = (256. * intensity.clamp(g)) as u32;
    let bbyte = (256. * intensity.clamp(b)) as u32;

    print!(
        "{rbyte} {gbyte} {bbyte}\n",
        rbyte = rbyte,
        gbyte = gbyte,
        bbyte = bbyte
    );
}
