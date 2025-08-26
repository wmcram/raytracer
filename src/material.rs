use std::sync::Arc;

use crate::{
    color::Color,
    hit::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::random_f64,
    vec3::{Vec3, dot, unit_vector},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Color {
        Color::default()
    }
}

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::from(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction).with_time(r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, rec.p);
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal);
        let reflected = unit_vector(reflected) + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(rec.p, reflected).with_time(r_in.time());
        *attenuation = self.albedo;

        return dot(scattered.direction(), rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    // The refractive index in vaccuum, or the ratio
    // of the material's refractive index over the refractive index
    // of its surrounding medium.
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_f64() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction).with_time(r_in.time());
        return true;
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_color(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::from(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.tex.value(u, v, p)
    }
}
