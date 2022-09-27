use crate::aabb::AABB;
use crate::material::Material;
use crate::rtweekend::PI;
use crate::{Point3, Ray, Vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material_ptr: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            point: Vec3::new(None, None, None),
            normal: Vec3::new(None, None, None),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material_ptr: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f32,
    material_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material_ptr,
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f32, v: &mut f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f32::acos(-p.y());
        let phi = f32::atan2(-p.z(), p.x()) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
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
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);
        hit_record.material_ptr = Some(self.material_ptr.clone());

        return true;
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center
                - Vec3 {
                    e: [self.radius, self.radius, self.radius],
                },
            self.center
                + Vec3 {
                    e: [self.radius, self.radius, self.radius],
                },
        );
        true
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub(crate) fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(hittable)
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    pub(crate) fn new(hittable: Option<Rc<dyn Hittable>>) -> HittableList {
        let mut new_list = HittableList { objects: vec![] };

        match hittable {
            None => {}
            Some(hittable) => {
                new_list.add(hittable);
            }
        }
        return new_list;
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            if i.hit(ray, t_min, closest_so_far, &mut temp) {
                hit_anything = true;
                closest_so_far = temp.t.clone();
                *hit_record = temp.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new(Vec3::new(None, None, None), Vec3::new(None, None, None));
        let mut first_box = true;

        for obj in &self.objects {
            if !obj.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }

        true
    }
}
