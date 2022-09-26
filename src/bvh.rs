use std::rc::Rc;
use crate::aabb::AABB;
use crate::Hittable;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB
}

impl BvhNode {

}