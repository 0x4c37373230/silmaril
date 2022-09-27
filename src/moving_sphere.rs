use crate::aabb::AABB;
use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};
use std::rc::Rc;

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        mat_ptr: Rc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center(ray.time())) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material_ptr = Some(self.mat_ptr.clone());

        return true;
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(Some(self.radius), Some(self.radius), Some(self.radius)),
            self.center(time0) + Vec3::new(Some(self.radius), Some(self.radius), Some(self.radius)),
        );
        let box1 = box0.clone();
        *output_box = AABB::surrounding_box(box0, box1);

        true
    }
}
