#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            x => x,
        }
    }

    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };
}
