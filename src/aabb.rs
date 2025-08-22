use crate::{interval::Interval, ray::Ray, vec3::Vec3};

#[derive(Default, Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    // Determines whether the given ray hits this bounding box
    // during the given interval.
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 > ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 > ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        return true;
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub const EMPTY: AABB = AABB::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);
    pub const UNIVERSE: AABB =
        AABB::new(Interval::UNIVERSE, Interval::UNIVERSE, Interval::UNIVERSE);
}

impl From<(Vec3, Vec3)> for AABB {
    fn from(v: (Vec3, Vec3)) -> Self {
        Self {
            x: if v.0.x() <= v.1.x() {
                Interval::new(v.0.x(), v.1.x())
            } else {
                Interval::new(v.1.x(), v.0.x())
            },
            y: if v.0.y() <= v.1.y() {
                Interval::new(v.0.y(), v.1.y())
            } else {
                Interval::new(v.1.y(), v.0.y())
            },
            z: if v.0.z() <= v.1.z() {
                Interval::new(v.0.z(), v.1.z())
            } else {
                Interval::new(v.1.z(), v.0.z())
            },
        }
    }
}

impl From<(AABB, AABB)> for AABB {
    fn from(v: (AABB, AABB)) -> Self {
        AABB::new(
            Interval::enclosing(v.0.x, v.1.x),
            Interval::enclosing(v.0.y, v.1.y),
            Interval::enclosing(v.0.z, v.1.z),
        )
    }
}
