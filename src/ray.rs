use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin() -> Vec3 {
        self.orig
    }

    pub fn direction() -> Vec3 {
        self.dir
    }

    pub fn at(t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}
