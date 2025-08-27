use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{Hit, HitRecord, Hittables},
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
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);
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
        let denom = dot(self.normal, r.direction());
        if f64::abs(denom) < 1e-8 {
            return false;
        }

        let t = (self.d - dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

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

// Makes an instance of a box with the given corners and material, represented as
// a hittable list of quads.
pub fn MakeBox(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Arc<Hittables> {
    let mut sides = Hittables::default();
    let mn = Vec3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let mx = Vec3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(mx.x() - mn.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, mx.y() - mn.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, mx.z() - mn.z());

    sides.add(Arc::new(Quad::new(
        Vec3::new(mn.x(), mn.y(), mx.z()),
        dx,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(mx.x(), mn.y(), mx.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(mx.x(), mn.y(), mn.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(mn.x(), mn.y(), mn.z()),
        dz,
        dy,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(mn.x(), mx.y(), mx.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Vec3::new(mn.x(), mn.y(), mn.z()),
        dx,
        dz,
        mat.clone(),
    )));
    Arc::new(sides)
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
