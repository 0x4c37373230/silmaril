use crate::{Hittable, Point3, Ray, Vec3};
use std::cmp::Ordering;
use std::mem::swap;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct AABB {
    pub maximum: Point3,
    pub minimum: Point3,
}

impl AABB {
    pub fn new(maximum: Point3, minimum: Point3) -> AABB {
        AABB { maximum, minimum }
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum
    }
    pub fn min(&self) -> &Point3 {
        &self.minimum
    }

    pub fn hit(&self, ray: &Ray, mut t_max: f32, mut t_min: f32) -> bool {
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

    pub fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis: i32) -> Ordering {
        let mut box_a = AABB::new(Vec3::new(None, None, None), Vec3::new(None, None, None));
        let mut box_b = box_a.clone();

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        box_a.min().e[axis as usize]
            .partial_cmp(&box_b.min().e[axis as usize])
            .unwrap()
    }

    pub fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}
