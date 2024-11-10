use crate::utils::interval::Interval;

use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

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
