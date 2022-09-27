use std::cmp::Ordering;
use crate::aabb::AABB;
use crate::{random, HitRecord, Hittable, HittableList, Ray, Vec3};
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    fn from(list: &HittableList, time0: f32, time1: f32) -> BvhNode {
        return BvhNode::new(&list.objects, 0, list.objects.len(), time0, time1);
    }

    fn new(
        src_objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BvhNode {
        let mut objects = src_objects.clone();
        let axis = random::<i32>(0, 3);
        type Comparator = fn(Rc<dyn Hittable>, Rc<dyn Hittable>) -> Ordering;
        let comparator: Comparator = if axis == 0 {
            AABB::box_x_compare
        } else if axis == 1 {
            AABB::box_y_compare
        } else {
            AABB::box_z_compare
        };
        let object_span = end - start;
        let temp_left: Rc<dyn Hittable>;
        let temp_right: Rc<dyn Hittable>;

        if object_span == 1 {
            temp_left = objects[start].clone();
            temp_right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(objects[start].clone(), objects[start + 1].clone()) == Ordering::Less {
                temp_left = objects[start].clone();
                temp_right = objects[start + 1].clone();
            } else {
                temp_left = objects[start + 1].clone();
                temp_right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| comparator(a.clone(), b.clone()));
            let mid = start + object_span / 2;
            temp_left = Rc::new(Self::new(&objects, start, mid, time0, time1));
            temp_right = Rc::new(Self::new(&objects, mid, end, time0, time1))
        }

        let mut box_left = AABB::new(Vec3::new(None, None, None), Vec3::new(None, None, None));
        let mut box_right = box_left.clone();

        if !temp_left.bounding_box(time0, time1, &mut box_left)
            || !temp_right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        BvhNode {
            left: temp_left,
            right: temp_right,
            bbox: AABB::surrounding_box(box_left, box_right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, hit_record);
        let hit_right = self.right.hit(
            ray,
            t_min,
            if hit_left { hit_record.t } else { t_max },
            hit_record,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;

        true
    }
}
