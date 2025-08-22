use std::sync::Arc;

use crate::hit::Hit;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center, Vec3::ZERO),
            radius: f64::max(0.0, radius),
            mat,
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut crate::hit::HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        // find nearest root in (tmin, tmax)
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.mat = self.mat.clone();
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}
