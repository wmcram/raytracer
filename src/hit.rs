use std::{default, sync::Arc};

use crate::{
    aabb::AABB,
    color::Color,
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Vec3, dot},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f64::default(),
            front_face: bool::default(),
            mat: Arc::new(Lambertian::new_color(Color::default())),
            u: Default::default(),
            v: Default::default(),
        }
    }
}

impl HitRecord {
    // Sets the hit record normal vector. The parameter outward_normal is assumed to have unit
    // length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hit: Send + Sync {
    // Determines if the given ray will hit the implementer such that t lies in
    // the interval (ray_tmin, ray_tmax). If so, the HitRecord struct will be populated
    // with information about the hit.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    // Returns an axis-aligned bounding box surrounding this object.
    fn bounding_box(&self) -> AABB;
}

#[derive(Default)]
pub struct Hittables {
    pub objects: Vec<Arc<dyn Hit>>,

    bbox: AABB,
}

impl Hittables {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hit>) {
        self.objects.push(object.clone());
        self.bbox = AABB::from((self.bbox, object.bounding_box()))
    }
}

impl<T: Hit + 'static> From<Arc<T>> for Hittables {
    fn from(value: Arc<T>) -> Self {
        Self {
            objects: vec![value],
            bbox: AABB::default(),
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct Translated {
    object: Arc<dyn Hit>,
    offset: Vec3,
    bbox: AABB,
}

impl Translated {
    pub fn new(object: Arc<dyn Hit>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hit for Translated {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction()).with_time(r.time());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;
        true
    }
}

pub struct Rotated {
    object: Arc<dyn Hit>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl Rotated {
    // Creates a new rotated instance of the given object with the given angle (in degrees!)
    pub fn new(object: Arc<dyn Hit>, angle: f64) -> Self {
        let rads = f64::to_radians(angle);
        let sin_theta = f64::sin(rads);
        let cos_theta = f64::cos(rads);
        let bbox = object.bounding_box();

        let mut mn = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut mx = -mn;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        mn[c] = f64::min(mn[c], tester[c]);
                        mx[c] = f64::max(mx[c], tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::from((mn, mx)),
        }
    }
}

impl Hit for Rotated {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction).with_time(r.time());
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Vec3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );
        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
