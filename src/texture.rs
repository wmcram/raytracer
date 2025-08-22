use std::sync::Arc;

use crate::{color::Color, vec3::Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl From<Color> for SolidColor {
    fn from(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl From<(f64, f64, f64)> for SolidColor {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            albedo: Color::new(value.0, value.1, value.2),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_solid(scale: f64, color1: Color, color2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::from(color1)),
            Arc::new(SolidColor::from(color2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x_int = f64::floor(self.inv_scale * p.x()) as i32;
        let y_int = f64::floor(self.inv_scale * p.y()) as i32;
        let z_int = f64::floor(self.inv_scale * p.z()) as i32;
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
