use crate::{Color, Point3};
use std::rc::Rc;
use crate::perlin::Perlin;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub(crate) struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Option<Color>) -> SolidColor {
        SolidColor {
            color_value: color_value.unwrap_or(Color::new(None, None, None)),
        }
    }

    fn from(r: f32, g: f32, b: f32) -> SolidColor {
        return Self::new(Some(Color::new(Some(r), Some(g), Some(b))));
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Point3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }

    pub fn from(color1: Color, color2: Color) -> CheckerTexture {
        CheckerTexture {
            even: Rc::new(SolidColor::new(Some(color1))),
            odd: Rc::new(SolidColor::new(Some(color2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Point3 {
        let sines = f32::sin(10.0 * p.x()) * f32::sin(10.0 * p.y()) * f32::sin(10.0 * p.z());

        return if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        };
    }
}

pub struct NoiseTexture {
    noise: Perlin
}

impl NoiseTexture {
    pub fn empty() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new()
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Point3 {
        Color::new(Some(1.0),Some(1.0),Some(1.0)) * self.noise.noise(p)
    }
}