use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double(float1: Option<f32>, float2: Option<f32>) -> f32 {
    let temp_float_a = float1.unwrap_or(0.0);
    let temp_float_b = float2.unwrap_or(1.0);

    let mut rng = rand::thread_rng();

    rng.gen_range(temp_float_a..temp_float_b)
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
