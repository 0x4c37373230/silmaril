use crate::aabb::AABB;
use crate::{
    HitRecord, Hittable, HittableList, Material, Point3, Ray, Vec3, XYRect, XZRect, YZRect,
};
use std::rc::Rc;

pub struct Block {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Block {
    pub fn new(point0: &Point3, point1: &Point3, material_ptr: Rc<dyn Material>) -> Block {
        let box_min = point0.clone();
        let box_max = point1.clone();
        let mut sides = HittableList::new(None);
        sides.add(Rc::new(XYRect::new(
            point0.x(),
            point1.x(),
            point0.y(),
            point1.y(),
            point1.z(),
            Some(material_ptr.clone()),
        )));
        sides.add(Rc::new(XYRect::new(
            point0.x(),
            point1.x(),
            point0.y(),
            point1.y(),
            point0.z(),
            Some(material_ptr.clone()),
        )));

        sides.add(Rc::new(XZRect::new(
            point0.x(),
            point1.x(),
            point0.z(),
            point1.z(),
            point1.y(),
            Some(material_ptr.clone()),
        )));
        sides.add(Rc::new(XYRect::new(
            point0.x(),
            point1.x(),
            point0.z(),
            point1.z(),
            point0.y(),
            Some(material_ptr.clone()),
        )));

        sides.add(Rc::new(YZRect::new(
            point0.y(),
            point1.y(),
            point0.z(),
            point1.z(),
            point1.x(),
            Some(material_ptr.clone()),
        )));
        sides.add(Rc::new(YZRect::new(
            point0.y(),
            point1.y(),
            point0.z(),
            point1.z(),
            point0.x(),
            Some(material_ptr),
        )));

        Block {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Block {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hit_record)
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_max, self.box_min);

        true
    }
}
