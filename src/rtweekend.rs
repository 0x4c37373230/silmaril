use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(
    start: T,
    end: T,
) -> T {
    let mut rng = rand::thread_rng();

    rng.gen_range(start..end)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    return if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    };
}
