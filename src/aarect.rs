use crate::aabb::AABB;
use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};
use std::rc::Rc;

pub struct XYRect {
    material_ptr: Option<Rc<dyn Material>>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn new(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material_ptr: Option<Rc<dyn Material>>,
    ) -> XYRect {
        XYRect {
            material_ptr,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().z()) / ray.direction().z();

        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (y - self.y0) / (self.y1 - self.y0);
        hit_record.t = t;
        let outward_normal = Vec3::new(None, None, Some(1.0));
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material_ptr = self.material_ptr.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(Some(self.x0), Some(self.y0), Some(self.k - 0.0001)),
            Point3::new(Some(self.x1), Some(self.y1), Some(self.k + 0.0001)),
        );

        return true;
    }
}
