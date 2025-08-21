use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(w: &mut impl Write, color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let intensity = Interval {
        min: 0.0,
        max: 0.999,
    };
    let rbyte = (255.999 * intensity.clamp(r)) as u8;
    let gbyte = (255.999 * intensity.clamp(g)) as u8;
    let bbyte = (255.999 * intensity.clamp(b)) as u8;

    write!(w, "{rbyte} {gbyte} {bbyte}\n").unwrap();
}
