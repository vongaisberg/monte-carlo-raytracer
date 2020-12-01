use crate::primitives::vec::Color;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::iter::{Flatten, Iterator};
use std::ops::Add;
use std::sync::{Arc, Mutex};

///A dot with a floating point location on a raw image
#[derive(Debug, Copy, Clone)]
pub struct RawDot {
    pub x: f64,
    pub y: f64,
    pub color: Color,
}

impl RawDot {
    pub fn new(x: f64, y: f64, color: Color) -> RawDot {
        RawDot {
            x: x,
            y: y,
            color: color,
        }
    }
}

/// A represtation of a pixel that preserves the exact position and color of every ray
#[derive(Debug, Clone)]
pub struct RawPixel {
    pub x: usize,
    pub y: usize,
    pub dots: Vec<RawDot>,
}

impl RawPixel {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            dots: Vec::new(),
        }
    }
    pub fn color(&self) -> Color {
        self.dots
            .iter()
            .map(|dot| dot.color)
            .fold(Color::BLACK, |sum, c| sum + c.pow(2f64))
            / (self.dots.len() as f64)
    }
    pub fn add_dot(&mut self, dot: RawDot) {
        self.dots.push(dot);
    }

    pub fn finalize(&mut self) {
        self.dots = vec![RawDot::new(self.x as f64, self.y as f64, self.color())];
    }

    pub fn std_div(&self) -> Color{
        let color = self.color();
        (self.dots
            .iter()
            .map(|dot| (dot.color - color).pow(2f64))
            .fold(Color::BLACK, Add::add)
            / (self.dots.len() as f64)).pow(0.5)
    }
}

//A representation of an image consisting of raw pixels
#[derive(Debug, Clone)]
pub struct RawImage {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<RawPixel>>,
}

impl RawImage {
    pub fn new(width: usize, height: usize) -> Self {
        let mut vec = Vec::with_capacity(width);

        for i in 0..width {
            vec.push(Vec::with_capacity(height));

            for j in 0..height {
                vec[i].push(RawPixel::new(i, j));
            }
        }
        Self {
            width: width,
            height: height,
            pixels: vec,
        }
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> &mut RawPixel {
        &mut self.pixels[x][y]
    }

    pub fn par_iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, std::vec::Vec<RawPixel>>> {
        self.pixels.iter().flatten()
    }
}
