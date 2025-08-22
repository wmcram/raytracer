use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        f64::sqrt(linear_component)
    } else {
        0.0
    }
}

pub fn write_color(w: &mut impl Write, color: &Color) {
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    let intensity = Interval {
        min: 0.0,
        max: 0.999,
    };
    let rbyte = (255.999 * intensity.clamp(r)) as u8;
    let gbyte = (255.999 * intensity.clamp(g)) as u8;
    let bbyte = (255.999 * intensity.clamp(b)) as u8;

    write!(w, "{rbyte} {gbyte} {bbyte}\n").unwrap();
}
