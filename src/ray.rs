use crate::vec3::Vec3;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn with_time(mut self, tm: f64) -> Self {
        self.tm = tm;
        self
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
