use crate::objects::traits::Intersect;
use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Color;
use std::cmp::Ordering::Equal;
use crate::objects::Object;
use crate::materials::Material;
use crate::cameras::Camera;



const EPSILON: f64 = f64::MIN_POSITIVE *10000f64;

pub struct Scene<'a> {
    objects: Vec<Box<dyn Object + Sync>>,
    pub camera: &'a (dyn Camera + Sync),
    pub sky_color: Color,
    ray_shooting_offset: f64,
}

impl<'a> Scene<'a> {

    pub fn new(camera: &'a (dyn Camera + Sync), sky_color: Color, ray_shooting_offset: f64) -> Scene<'a> {
        Scene{
            objects: Vec::new(),
            camera: camera,
            sky_color: sky_color,
            ray_shooting_offset: ray_shooting_offset,
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object + Sync>) {
        self.objects.push(obj);
    }

    pub fn trace_ray(&self, ray: &Ray, max_depth: u64) -> Color {
        if max_depth == 0 {
            //println!("Depth exhaustion");
            Color::BLACK
        } else {
            if let Some(intersection) = self.shoot_ray(ray) {
                let (recursive_ray, reflective_color, additive_color) =
                    intersection.object.material().scatter(ray, &intersection);
                let recursive_ray = Ray::new(recursive_ray.origin + recursive_ray.direction* EPSILON, recursive_ray.direction);
                let color = additive_color + (reflective_color * self.trace_ray(&recursive_ray, max_depth - 1));
                //println!("Depth: {}, Color: {:?}", max_depth, color);
                color
            } else {
                self.sky_color
            }
        }
    }

    fn shoot_ray(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|obj| obj.intersect(&ray, self.ray_shooting_offset))
            .min_by(|a, b| {
                a.ray_parameter
                    .partial_cmp(&b.ray_parameter)
                    .unwrap_or(Equal)
            })
    }
}
