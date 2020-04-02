pub mod raw;
pub mod fixed_samples;
pub mod std_div_renderer;
pub mod combined_renderer;

use crate::objects::scene::Scene;
use crate::renderer::raw::RawImage;

pub trait Renderer<'a> {
    fn new(scene: &'a Scene) -> Self;
    fn render(&self, raw_image: &mut RawImage);
}