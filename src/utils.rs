use rand::Rng;

// Samples a random f64 from [0,1).
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

// Samples a random f64 from [min,max)
pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
