use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn size(&self) -> f64 {
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

    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    // Returns the interval enclosing both i1 and i2.
    pub fn enclosing(i1: Interval, i2: Interval) -> Self {
        Self {
            min: f64::min(i1.min, i2.min),
            max: f64::max(i1.max, i2.max),
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

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}

// Adding an f64 to an Interval displaces the Interval accordingly.
impl Add<f64> for Interval {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.min + rhs, self.max + rhs)
    }
}

// Adding an f64 to an Interval displaces the Interval accordingly.
impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}
