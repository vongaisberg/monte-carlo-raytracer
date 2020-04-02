use crate::primitives::ray::Ray;
use crate::primitives::vec::Vector;
use crate::cameras::Camera;

pub struct Pinhole {
    pub viewpoint: Vector,
    viewplane_top_left: Vector,
    viewplane_down: Vector,
    viewplane_right: Vector,
}

impl Pinhole {
    pub fn new(
        viewpoint: Vector,
        viewplane_top_left: Vector,
        viewplane_down: Vector,
        viewplane_right: Vector,
    ) -> Self {
        Self {
            viewpoint: viewpoint,
            viewplane_top_left: viewplane_top_left,
            viewplane_down: viewplane_down,
            viewplane_right: viewplane_right,
        }
    }
}
impl Camera for Pinhole {
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let viewplane_point =
            self.viewplane_top_left + (self.viewplane_down * y) + (self.viewplane_right * x);
        Ray::new(self.viewpoint, viewplane_point - self.viewpoint)
    }
}
