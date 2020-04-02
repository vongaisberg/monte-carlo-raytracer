use crate::objects::Object;
use crate::objects::traits::Intersect;
use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Vector;
use std::option::Option;
use crate::materials::Material;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Object for Sphere {
    fn material(&self) -> &Box<dyn Material>{
        &self.material
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray, param_min: f64) -> Option<Intersection> {
        let a = ray.direction.dot(&ray.direction);
        let b = (ray.direction * 2f64).dot(&(ray.origin - self.center));
        let c = (ray.origin - self.center).dot(&(ray.origin - self.center))
            - (self.radius * self.radius);

        let discriminant = (b * b) - (4f64 * a * c);
        if discriminant < 0f64 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 < t2 {
                if t1 > param_min {
                    let intersection_position = ray.point_at_parameter(t1);
                    let intersection_normal = (intersection_position - self.center).normalize();
                    Some(Intersection::new(
                        intersection_position,
                        self as &dyn Object,
                        t1,
                        intersection_normal,
                    ))
                } else if t2 > param_min {
                    let intersection_position = ray.point_at_parameter(t2);
                    let intersection_normal = (intersection_position - self.center).normalize();
                    Some(Intersection::new(
                        intersection_position,
                        self as &dyn Object,
                        t2,
                        intersection_normal,
                    ))
                } else {
                    None
                }
            } else {
                if t2 > param_min {
                    let intersection_position = ray.point_at_parameter(t2);
                    let intersection_normal = (intersection_position - self.center).normalize();
                    Some(Intersection::new(
                        intersection_position,
                        self as &dyn Object,
                        t2,
                        intersection_normal,
                    ))
                } else if t1 > param_min {
                    let intersection_position = ray.point_at_parameter(t1);
                    let intersection_normal = (intersection_position - self.center).normalize();
                    Some(Intersection::new(
                        intersection_position,
                        self as &dyn Object,
                        t1,
                        intersection_normal,
                    ))
                } else {
                    None
                }
            }
        }
    }
}
