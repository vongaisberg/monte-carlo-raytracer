use crate::materials::Material;
use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::{Color, Vector};

#[cfg(test)]
use crate::objects::sphere::Sphere;


use rand::Rng;

pub struct PseudoPhong {
    spectral_term: f64,
    spectral_fuzziness: f64,

    reflection_color: Color,
    radiation_color: Color,
}
impl PseudoPhong {
    pub fn new(
        spectral_term: f64,
        spectral_fuzziness: f64,
        reflection_color: Color,
        radiation_color: Color,
    ) -> Self {
        Self {
            spectral_term: spectral_term,
            spectral_fuzziness: spectral_fuzziness,
            reflection_color: reflection_color,
            radiation_color: radiation_color,
        }
    }
}

impl Material for PseudoPhong {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> (Ray, Color, Color) {
        let mut rng = rand::thread_rng();
        let direction = if rng.gen::<f64>() < self.spectral_term {
            ray.direction.reflect(&intersection.normal).normalize()
                + Vector::random_on_unit_sphere() * self.spectral_fuzziness
        } else {
           // println!("Diffuse");
            intersection.normal + Vector::random_on_unit_sphere()
        };
        (
            Ray::new(intersection.position, direction),
            self.reflection_color,
            self.radiation_color,
        )
    }
}

#[test]
fn test_scatter() {

    let phong = PseudoPhong::new(
        1f64,
        0.2,
        Color::new(100f64, 0f64, 0f64),
        Color::new(0f64, 0f64, 0f64),
    );

    let sphere = Sphere::new(Vector::new(0f64, 0f64, 0f64), 1f64, Box::new(phong));

    let intersection = Intersection::new(Vector::new(1f64, 0f64, 0f64),  &sphere, 1f64, Vector::new(1f64, 0f64, 0f64));

  //  println!("TEST: {:?}", sphere.material().scatter(&Ray::new(Vector::new(3f64, 0f64, 0f64), Vector::new(-1f64, 0f64, 0f64)), &intersection));

}