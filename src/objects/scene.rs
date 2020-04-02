use crate::objects::traits::Intersect;
use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Color;
use std::cmp::Ordering::Equal;
use crate::objects::Object;
use crate::materials::Material;
use crate::cameras::Camera;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    pub camera: Box<Camera>,
    sky_color: Color,
    ray_shooting_offset: f64,
}

impl Scene {

    pub fn new(camera: Box<Camera>, sky_color: Color, ray_shooting_offset: f64) -> Scene {
        Scene{
            objects: Vec::new(),
            camera: camera,
            sky_color: sky_color,
            ray_shooting_offset: ray_shooting_offset,
        }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
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
