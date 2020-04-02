use crate::primitives::vec::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
