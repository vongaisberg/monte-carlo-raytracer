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
use threadpool::ThreadPool;

pub struct CombinedRenderer<'a> {
    scene: &'a Scene,
    first_stage_samples_per_pixel: usize,
    second_stage_samples_per_pixel: usize,
    min_std_div: Color,
}

impl<'a> CombinedRenderer<'a> {
    pub fn first_stage_set_samples_per_pixel(&mut self, samples: usize) {
        self.first_stage_samples_per_pixel = samples;
    }
    pub fn second_stage_set_samples_per_pixel(&mut self, samples: usize) {
        self.second_stage_samples_per_pixel = samples;
    }
    pub fn set_min_std_div(&mut self, std_div: Color) {
        self.min_std_div = std_div;
    }
}

impl<'a> Renderer<'a> for CombinedRenderer<'a> {
    fn new(scene: &'a Scene) -> Self {
        Self {
            scene: scene,
            first_stage_samples_per_pixel: 1,
            second_stage_samples_per_pixel: 1,
            min_std_div: Color::BLACK,
        }
    }

    fn render(&self, img: &mut RawImage) {
        //let pool = ThreadPool::new(7);
        let max_depth = 50;
        let raster_size = (self.first_stage_samples_per_pixel as f64).sqrt() as usize;
        let raster_width: f64 = 1f64 / (raster_size as f64);
        let mut rng = thread_rng();

        let mut changed_pixels = 0;
        let mut unchanged_pixels = 0;

        for pixel_x in 0..img.width {
            for pixel_y in 0..img.height {
                if pixel_y == 0 {
                    println!("Pixel: {} {}", pixel_x, pixel_y);
                }
                let mut pixel = RawPixel::new(pixel_x, pixel_y);

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

                        pixel.add_dot(RawDot::new(x, y, self.scene.trace_ray(&ray, max_depth)));
                    }
                }

                if !(pixel.color() == self.min_std_div) {
                    changed_pixels += 1;
                    let raster_size = (self.second_stage_samples_per_pixel as f64).sqrt() as usize;
                    let raster_width: f64 = 1f64 / (raster_size as f64);

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

                            pixel.add_dot(RawDot::new(x, y, self.scene.trace_ray(&ray, max_depth)));
                        }
                    }
                } else {
                    unchanged_pixels += 1;
                }
                let color = pixel.color();
                img.pixel(pixel_x, pixel_y).add_dot(RawDot::new(
                    pixel_x as f64,
                    pixel_y as f64,
                    color,
                ));
            }
        }

        println!(
            "unchanged pixels: {}, changed pixels: {}",
            unchanged_pixels, changed_pixels
        );
    }
}
