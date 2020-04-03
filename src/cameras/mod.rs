use crate::primitives::ray::Ray;

pub mod pinhole;
pub mod thin_lense;

pub trait Camera {
    fn get_ray(&self, x: f64, y: f64) -> Ray;
}