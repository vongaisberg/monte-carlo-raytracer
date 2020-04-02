extern crate image;
use crate::renderer::raw::{RawImage, RawPixel};

use std::path::Path;

pub fn gen_ppm(img: &mut RawImage, filename: String) -> () {
    // Time to write to image file!
    let path = Path::new(&filename);
    let display = path.display();
    let sizey = img.height;
    let sizex = img.width;
    let mut imgbuf = image::ImageBuffer::new(sizex as u32, sizey as u32);

    for x in 0..sizex {
        for y in 0..sizey {
            let color = img.pixel(x, y).color();
            imgbuf.put_pixel(
                x as u32,
                y as u32,
                image::Rgb([
                    (color.r().powf(0.5) * 255f64) as u8,
                    (color.g().powf(0.5) * 255f64) as u8,
                    (color.b().powf(0.5) * 255f64) as u8,
                ]),
            );
        }
    }

    let _ = image::DynamicImage::ImageRgb8(imgbuf).save(&path);
    println!("successfully wrote to {}", display);
}
