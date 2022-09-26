use crate::{Point3, Ray, Vec3};
use std::cmp;
use std::mem::swap;

#[derive(Clone, Copy)]
pub struct AABB {
    maximum: Point3,
    minimum: Point3,
}

impl AABB {
    pub(crate) fn new(maximum: Point3, minimum: Point3) -> AABB {
        AABB { maximum, minimum }
    }

    fn max(&self) -> &Point3 {
        &self.maximum
    }
    fn min(&self) -> &Point3 {
        &self.minimum
    }

    fn hit(&self, ray: &Ray, mut t_max: f32, mut t_min: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction().x();
            let mut t0 = (self.min().x() - ray.origin().x()) * inv_d;
            let mut t1 = (self.max().x() - ray.origin().x()) * inv_d;

            if inv_d < 0.0 {
                swap(&mut t0, &mut t1)
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            Some(f32::min(box0.min().x(), box1.min().x())),
            Some(f32::min(box0.min().y(), box1.min().y())),
            Some(f32::min(box0.min().z(), box1.min().z())),
        );
        let big = Vec3::new(
            Some(f32::max(box0.max().x(), box1.max().x())),
            Some(f32::max(box0.max().y(), box1.max().y())),
            Some(f32::max(box0.max().z(), box1.max().z())),
        );

        AABB::new(small, big)
    }
}
