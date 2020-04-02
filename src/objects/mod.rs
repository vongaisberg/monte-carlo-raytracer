pub mod sphere;
pub mod traits;
pub mod scene;

use crate::materials::Material;
use crate::objects::traits::Intersect;

pub trait Object: Intersect {
    fn material(&self) -> &Box<dyn Material>;
}