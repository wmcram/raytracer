use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord},
    interval::Interval,
    material::Material,
    vec3::{Vec3, cross, dot, unit_vector},
};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        let mut quad = Quad {
            q,
            u,
            v,
            normal,
            d,
            w,
            mat,
            bbox: AABB::default(),
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let diag1 = AABB::from((self.q, self.q + self.u + self.v));
        let diag2 = AABB::from((self.q + self.u, self.q + self.v));
        self.bbox = AABB::from((diag1, diag2));
    }
}

impl Hit for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hit::HitRecord,
    ) -> bool {
        let denom = dot(&self.normal, &r.direction());
        if f64::abs(denom) < 1e-8 {
            return false;
        }

        let t = (self.d - dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !is_interior(alpha, beta, rec) {
            return false;
        }

        // we have a hit, fill in hitrec
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);
    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }
    rec.u = a;
    rec.v = b;
    true
}
