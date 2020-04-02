use crate::objects::Object;
use crate::primitives::vec::Vector;

pub struct Intersection<'a> {
    pub position: Vector,
    pub object: &'a dyn Object,
    pub ray_parameter: f64,
    pub normal: Vector,
}

impl<'a> Intersection<'a> {
    pub fn new(position: Vector, object: &'a dyn Object, ray_parameter: f64, normal: Vector) -> Self {
        Self {
            position: position,
            object: object,
            ray_parameter: ray_parameter,
            normal: normal,
        }
    }
}
