use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;

pub trait Intersect {
    fn intersect(&self, ray: &Ray, param_min: f64) -> Option<Intersection>;
}
