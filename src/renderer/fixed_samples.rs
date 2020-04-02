use super::{
    raw::{RawDot, RawImage, RawPixel},
    Renderer,
};
use crate::cameras::Camera;
use crate::objects::scene::Scene;
use crate::primitives::ray::Ray;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub struct FixedSamplesRenderer<'a> {
    scene: &'a Scene,
    samples_per_pixel: usize,
}

impl<'a> FixedSamplesRenderer<'a> {
    pub fn set_samples_per_pixel(&mut self, samples: usize) {
        self.samples_per_pixel = samples;
    }
}

impl<'a> Renderer<'a> for FixedSamplesRenderer<'a> {
    fn new(scene: &'a Scene) -> Self {
        Self {
            scene: scene,
            samples_per_pixel: 1,
        }
    }

    fn render(&self, img: &mut RawImage) {
        let max_depth = 100;
        let raster_size = (self.samples_per_pixel as f64).sqrt() as usize;
        let raster_width: f64 = 1f64 / (raster_size as f64);
        let mut rng = thread_rng();
        for pixel_x in 0..img.width {
            for pixel_y in 0..img.height {
                if pixel_y == 0 {
                    println!("Pixel: {} {}", pixel_x, pixel_y);
                }
                for i in 0..raster_size {
                    let left = ((pixel_x as f64) + ((i as f64) * (raster_width as f64)))
                        / (img.width as f64);
                    //println!("X: {}, Left: {}", (pixel.x as f64 / img.width as f64), left);
                    for j in 0..raster_size {
                        let top = ((pixel_y as f64) + ((j as f64) * (raster_width as f64)))
                            / (img.height as f64);

                        //Generate a random point in that raster
                        let x: f64 = left + (rng.gen::<f64>() * raster_width / (img.width as f64));
                        let y: f64 = top + (rng.gen::<f64>() * raster_width / (img.height as f64));

                        let ray = self.scene.camera.get_ray(x, y);

                        img.pixel(pixel_x, pixel_y).add_dot(RawDot::new(
                            x,
                            y,
                            self.scene.trace_ray(&ray, max_depth),
                        ));
                    }
                }
                // pixel.finalize();
            }
        }
    }
}
