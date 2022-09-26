use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Option<Point3>, direction: Option<Vec3>, tm: Option<f32>) -> Ray {
        Ray {
            orig: origin.unwrap_or(Point3::new(None, None, None)),
            dir: direction.unwrap_or(Vec3::new(None, None, None)),
            time: tm.unwrap_or(0.0),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f32 {
        self.time
    }

    // 'origin + t * direction' would be the mathematical representation of a ray.
    // t can basically be any real number and using both positive and negative numbers you can
    // 'move' through a ray. Using positive values you only get parts in front of the origin. That's
    // called a half-line/ray
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
