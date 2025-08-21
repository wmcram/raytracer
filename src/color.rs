use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(w: &mut impl Write, color: &Color) {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    write!(w, "{rbyte} {gbyte} {bbyte}\n").unwrap();
}
