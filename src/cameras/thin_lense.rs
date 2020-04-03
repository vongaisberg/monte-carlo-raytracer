use crate::cameras::Camera;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Vector;

pub struct ThinLenseCamera {
    pub viewpoint: Vector,
    viewplane_top_left: Vector,
    viewplane_down: Vector,
    viewplane_right: Vector,
    focus_distance: f64,
    aperture: f64,
}

impl ThinLenseCamera {
    pub fn new(
        viewpoint: Vector,
        viewplane_top_left: Vector,
        viewplane_down: Vector,
        viewplane_right: Vector,
        focus_distance: f64,
        aperture: f64,
    ) -> Self {
        Self {
            viewpoint: viewpoint,
            viewplane_top_left: viewplane_top_left,
            viewplane_down: viewplane_down,
            viewplane_right: viewplane_right,
            focus_distance: focus_distance,
            aperture: aperture,
        }
    }
}
impl Camera for ThinLenseCamera {
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let mut blur_offset = Vector::random_in_unit_sphere() * self.aperture;

        let viewplane_point =
            self.viewplane_top_left + (self.viewplane_down * y) + (self.viewplane_right * x);

        let viewpoint = self.viewpoint + blur_offset;
        Ray::new(
            viewpoint,
            ((viewplane_point - self.viewpoint) * self.focus_distance) + self.viewpoint - viewpoint,
        )
    }
}
