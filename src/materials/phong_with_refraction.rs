use crate::materials::Material;
use crate::primitives::intersection::Intersection;
use crate::primitives::ray::Ray;
use crate::primitives::vec::{Color, Vector};


#[cfg(test)]
use crate::objects::sphere::Sphere;


use num::clamp;
use rand::Rng;

pub struct PseudoPhongRefraction {
    spectral_term: f64,
    spectral_fuzziness: f64,

    refraction_index: f64,

    reflection_color: Color,
    radiation_color: Color,
}
impl PseudoPhongRefraction {
    pub fn new(
        spectral_term: f64,
        spectral_fuzziness: f64,
        refraction_index: f64,
        reflection_color: Color,
        radiation_color: Color,
    ) -> Self {
        Self {
            spectral_term: spectral_term,
            spectral_fuzziness: spectral_fuzziness,
            refraction_index: refraction_index,
            reflection_color: reflection_color,
            radiation_color: radiation_color,
        }
    }
}

impl PseudoPhongRefraction {
    fn refract(&self, ray: &Ray, intersection: &Intersection) -> Option<Vector> {
        let cosI = clamp(intersection.normal.dot(&ray.direction), -1., 1.);
        let normal = intersection.normal;
        let (eta, cosI, normal) = if cosI < 0. {
            //The ray is coming from the outside
            (1. / self.refraction_index, -cosI, normal)
        } else {
            (self.refraction_index, cosI, -normal)
        };

        let k = 1. - eta * eta * (1. - cosI * cosI);

        if k < 0. {
            //Total reflection
            None
        } else {
            let direction = ray.direction * eta + normal * (eta * cosI - k.sqrt());
            Some(direction)
        }
    }

    fn fresnel(&self, ray: &Ray, intersection: &Intersection) -> f64 {
        let cosi = clamp(ray.direction.dot(&intersection.normal), -1., 1.);

        let (etai, etat) = if cosi > 0. {
            (self.refraction_index, 1.)
        } else {
            (1., self.refraction_index)
        };
        // Compute sini using Snell's law
        let sint = etai / etat * 0f64.max(1. - cosi * cosi).sqrt();
        // Total internal reflection
        if sint >= 1. {
            return 1.;
        } else {
            let cost = 0f64.max(1f64 - sint * sint).sqrt();
            let cosi = cosi.abs();
            let Rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
            let Rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
            return (Rs * Rs + Rp * Rp) / 2.;
        }
        // As a consequence of the conservation of energy, transmittance is given by:
        // kt = 1 - kr;
    }
}

impl Material for PseudoPhongRefraction {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> (Ray, Color, Color) {
        let mut rng = rand::thread_rng();

        //reflection_chance >= 1 is total internal reflection
        let reflection_chance = self.fresnel(ray, intersection);
        if reflection_chance != 0.04000000000000001 {
            //println!("Reflection_chance: {}", reflection_chance);
        }
        /**return (
            Ray::new(intersection.position, intersection.normal),
            Color::new(255f64 * reflection_chance, 0f64, 0f64),
            Color::new(255f64 * reflection_chance, 0f64, 0f64),
        );**/
        let val = rng.gen::<f64>();
        let direction = if val < reflection_chance {
            //Reflect the ray according to phong
            if rng.gen::<f64>() < self.spectral_term {
                //Spectral reflection

                return (
                    Ray::new(intersection.position, intersection.normal),
                    Color::new(255f64, 0f64, 0f64),
                    Color::new(255f64, 0f64, 0f64),
                );
            //ray.direction.reflect(&intersection.normal).normalize()
            //    + Vector::random_on_unit_sphere() * self.spectral_fuzziness
            } else {
                // Diffuse reflection
                intersection.normal + Vector::random_on_unit_sphere()
            }
        } else {
            // Refract the ray according to phong
            if rng.gen::<f64>() < self.spectral_term {
                self.refract(ray, intersection).unwrap().normalize()
                    + Vector::random_on_unit_sphere() * self.spectral_fuzziness
            } else {
                (-intersection.normal) + Vector::random_on_unit_sphere()
            }
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
    let phong = PseudoPhongRefraction::new(
        0.2f64,
        0.01f64,
        1.5f64,
        Color::new(100f64, 0f64, 0f64),
        Color::new(0f64, 0f64, 0f64),
    );

    let sphere = Sphere::new(Vector::new(0f64, 0f64, 0f64), 1f64, Box::new(phong));

    let intersection = Intersection::new(
        Vector::new(1f64, 0f64, 0f64),
        &sphere,
        1f64,
        Vector::new(1f64, 0f64, 0f64),
    );
/*
    println!(
        "TEST: {:?}",
        sphere.material().scatter(
            &Ray::new(
                Vector::new(3f64, 0f64, 0f64),
                Vector::new(-1f64, 0f64, 0f64)
            ),
            &intersection
        )
    );
    */
}
