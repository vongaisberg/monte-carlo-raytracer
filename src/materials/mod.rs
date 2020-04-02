pub mod phong;

use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Color;

pub trait Material {
    
    /// Scatter the incoming ray.
    /// 
    /// Returns a recursive ray, a reflective (mutiplicative) color and a irradiated (additive) color
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> (Ray, Color, Color);
}