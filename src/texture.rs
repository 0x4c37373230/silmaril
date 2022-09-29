use crate::perlin::Perlin;
use crate::{clamp, Color, Point3};
use stb_image::image::LoadResult;
use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub struct SolidColor {
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
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: Option<f32>) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: scale.unwrap_or(1.0),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Point3 {
        Color::new(Some(1.0), Some(1.0), Some(1.0))
            * 0.5
            * (1.0 + f32::sin(self.scale * p.z() + 10.0 * self.noise.turbulence(p, None)))
    }
}

pub struct ImageTexture {
    data: Option<Vec<u8>>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: i32 = 3;

    pub fn empty() -> ImageTexture {
        ImageTexture {
            data: None,
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn new(filename: &str) -> ImageTexture {
        let img_res = stb_image::image::load(filename);
        let temp_data = match img_res {
            LoadResult::Error(_) => return Self::empty(),
            LoadResult::ImageU8(image) => image,
            LoadResult::ImageF32(_) => return Self::empty(),
        };

        ImageTexture {
            data: Some(temp_data.data),
            width: temp_data.width as i32,
            height: temp_data.height as i32,
            bytes_per_scanline: Self::BYTES_PER_PIXEL * temp_data.width as i32,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Point3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.is_none() {
            return Color::new(None, Some(1.0), Some(1.0));
        }
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates
        let mut i = (u * self.width as f32) as i32;
        let mut j = (v * self.height as f32) as i32;
        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale: f32 = 1.0 / 255.0;
        let pixel = (j * self.bytes_per_scanline + i * Self::BYTES_PER_PIXEL) as usize;

        Color::new(
            Some(self.data.as_ref().unwrap()[pixel] as f32 * color_scale),
            Some(self.data.as_ref().unwrap()[pixel + 1] as f32 * color_scale),
            Some(self.data.as_ref().unwrap()[pixel + 2] as f32 * color_scale),
        )
    }
}
