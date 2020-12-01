use super::{
    raw::{RawDot, RawImage, RawPixel},
    Renderer,
};
use crate::cameras::Camera;
use crate::objects::scene::Scene;
use crate::primitives::ray::Ray;
use crate::primitives::vec::Color;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub struct StdDivRenderer<'a> {
    scene: &'a Scene<'a>,
    samples_per_pixel: usize,
    min_std_div: Color,
}

impl<'a> StdDivRenderer<'a> {
    pub fn set_samples_per_pixel(&mut self, samples: usize) {
        self.samples_per_pixel = samples;
    }
    pub fn set_min_std_div(&mut self, std_div: Color) {
        self.min_std_div = std_div;
    }
}

impl<'a> Renderer<'a> for StdDivRenderer<'a> {
    fn new(scene: &'a Scene) -> Self {
        Self {
            scene: scene,
            samples_per_pixel: 1,
            min_std_div: Color::BLACK,
        }
    }

    fn render(&self, img: &mut RawImage) {
        let max_depth = 100;
        let raster_size = (self.samples_per_pixel as f64).sqrt() as usize;
        let raster_width: f64 = 1f64 / (raster_size as f64);
        let mut rng = thread_rng();
        let mut changed_pixels = 0;
        let mut unchanged_pixels = 0;
        for pixel_x in 0..img.width {
            for pixel_y in 0..img.height {
                if pixel_y == 0 {
                    println!("StdDivPixel: {} {}", pixel_x, pixel_y);
                }
                if !img.pixel(pixel_x, pixel_y).lock().unwrap().std_div().less_than(self.min_std_div) {
                    changed_pixels += 1;

                    for i in 0..raster_size {
                        let left = ((pixel_x as f64) + ((i as f64) * (raster_width as f64)))
                            / (img.width as f64);
                        //println!("X: {}, Left: {}", (pixel.x as f64 / img.width as f64), left);
                        for j in 0..raster_size {
                            let top = ((pixel_y as f64) + ((j as f64) * (raster_width as f64)))
                                / (img.height as f64);

                            //Generate a random point in that raster
                            let x: f64 =
                                left + (rng.gen::<f64>() * raster_width / (img.width as f64));
                            let y: f64 =
                                top + (rng.gen::<f64>() * raster_width / (img.height as f64));

                            let ray = self.scene.camera.get_ray(x, y);

                            img.pixel(pixel_x, pixel_y).lock().unwrap().add_dot(RawDot::new(
                                x,
                                y,
                                self.scene.trace_ray(&ray, max_depth),
                            ));
                        }
                    }
                } else {
                    unchanged_pixels += 1;
                }
                 img.pixel(pixel_x, pixel_y).lock().unwrap().finalize();
            }
        }
        println!(
            "unchanged pixels: {}, changed pixels: {}",
            unchanged_pixels, changed_pixels
        );
        
    }
}
