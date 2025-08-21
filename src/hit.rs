use std::rc::Rc;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Vec3, dot},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Sets the hit record normal vector. The parameter outward_normal is assumed to have unit
    // length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hit {
    // Determines if the given ray will hit the implementer such that t lies in
    // the interval (ray_tmin, ray_tmax). If so, the HitRecord struct will be populated
    // with information about the hit.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct Hittables {
    pub objects: Vec<Rc<dyn Hit>>,
}

impl Hittables {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hit>) {
        self.objects.push(object);
    }
}

impl From<Rc<dyn Hit>> for Hittables {
    fn from(value: Rc<dyn Hit>) -> Self {
        Self {
            objects: vec![value],
        }
    }
}

impl Hit for Hittables {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in self.objects.iter() {
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
}
