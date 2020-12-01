extern crate raytracer;

use rand::{thread_rng, Rng};
use raytracer::cameras::pinhole::Pinhole;
use raytracer::cameras::thin_lense::ThinLenseCamera;
use raytracer::cameras::Camera;
use raytracer::io::export;
use raytracer::materials::phong::PseudoPhong;
use raytracer::materials::phong_with_refraction::PseudoPhongRefraction;
use raytracer::objects::{scene::Scene, sphere::Sphere};
use raytracer::primitives::vec::{Color, Vector};
use raytracer::renderer::combined_renderer::CombinedRenderer;
use raytracer::renderer::fixed_samples::FixedSamplesRenderer;
use raytracer::renderer::raw::RawImage;
use raytracer::renderer::std_div_renderer::StdDivRenderer;
use raytracer::renderer::Renderer;

use rayon::prelude::*;

fn main() {
    (0..1).into_par_iter().for_each(|focus_factor| {
        let camera = Pinhole::new(
            Vector::new(0f64, 2.5f64, 0f64),
            Vector::new(5f64, 5f64, 0f64),
            Vector::new(0f64, -5f64, 0f64),
            Vector::new(-5f64, 0f64, 5f64),
        );

        /*
        let camera = ThinLenseCamera::new(
            Vector::new(0f64, 2.5f64, 0f64),
            Vector::new(5f64, 5f64, 0f64),
            Vector::new(0f64, -5f64, 0f64),
            Vector::new(-5f64, 0f64, 5f64),
            (focus_factor as f64) / 25f64,
            1f64,
        );

        */
        let mut scene = Scene::new(
            &camera,
            Color::new(150f64 * 0.5, 170f64 * 0.5, 220f64 * 0.5),
            0.0001,
        );

        let material = PseudoPhong::new(
            0.9,
            0.1,
            Color::new(200f64, 70f64, 40f64),
            Color::new(0f64, 0f64, 0f64),
        );
        let material_sun = PseudoPhong::new(
            0.4,
            0.2,
            Color::new(100f64, 100f64, 100f64),
            Color::new(250f64 * 1., 250f64 * 0.8 * 1., 250f64 * 0.5 * 1.),
        );
        let material_sun2 = PseudoPhong::new(
            0.5,
            0.2,
            Color::new(0f64, 0f64, 0f64),
            Color::new(255f64 * 3., 255f64 * 3., 255f64 * 3.),
        );
        let material_metal = PseudoPhong::new(
            0.95f64,
            0f64,
            Color::new(250f64, 250f64, 250f64),
            Color::BLACK,
        );
        let material_ground = PseudoPhong::new(
            0.3f64,
            0.05f64,
            Color::new(255f64, 255f64, 255f64),
            Color::BLACK,
        );
        let material_matte = PseudoPhong::new(
            0.05f64,
            0.1f64,
            Color::new(180f64, 180f64, 250f64),
            Color::BLACK,
        );

        let sphere = Sphere::new(Vector::new(5f64, 3f64, 11f64), 3f64, Box::new(material));

        let sun = Sphere::new(
            Vector::new(24.5f64, 3f64, 11f64),
            3f64,
            Box::new(material_sun),
        );
        let sun2 = Sphere::new(
            Vector::new(30f64, 80f64, 50f64),
            25f64,
            Box::new(material_sun2),
        );

        let mirror = Sphere::new(
            Vector::new(11.5f64, 3f64, 11f64),
            3f64,
            Box::new(material_metal),
        );

        let ground = Sphere::new(
            Vector::new(0f64, -1001f64, 0f64),
            1000f64,
            Box::new(material_ground),
        );

        let matte = Sphere::new(
            Vector::new(18f64, 3f64, 11f64),
            3f64,
            Box::new(material_matte),
        );

        scene.add_object(Box::new(sphere));
        scene.add_object(Box::new(sun));
        scene.add_object(Box::new(mirror));
        scene.add_object(Box::new(sun2));
        scene.add_object(Box::new(ground));
        scene.add_object(Box::new(matte));

        let mut rng = thread_rng();
        /*
        for a in 0..10 {
            for b in 1..10 {
                    if a > 4 || b > 3  {
                        if rng.gen::<f64>() < 0.3 {
                            let center = Vector::new(
                                (a as f64) * 3f64,
                                0.01f64,
                                ((a as f64) * 0.4) + (b as f64) * 3f64,
                            );
                            let material = PseudoPhong::new(
                                rng.gen(),
                                rng.gen::<f64>() * 0.5,
                                Color::new(
                                    rng.gen::<f64>() * 255f64,
                                    rng.gen::<f64>() * 255f64,
                                    rng.gen::<f64>() * 255f64,
                                ),
                                Color::BLACK,
                            );
                            scene.add_object(Box::new(Sphere::new(center, 1f64, Box::new(material))));
                        }
                    }
            }
        }
        */

        //let mut renderer = FixedSamplesRenderer::new(&scene);
        //let mut std_div_renderer = StdDivRenderer::new(&scene);
        let mut combined_renderer = CombinedRenderer::new(&scene);
        //renderer.set_samples_per_pixel(250);
        //std_div_renderer.set_samples_per_pixel(2000);
        //combined_renderer.set_min_std_div(scene.sky_color);
        combined_renderer.first_stage_set_samples_per_pixel(2000);
        combined_renderer.second_stage_set_samples_per_pixel(15000);
        //combined_renderer.set_min_std_div(Color::new(0.05, 0.05, 0.05));

        let mut img = RawImage::new(150 * 30, 100 * 30);

        //renderer.render(&mut img);
        combined_renderer.render(&mut img);
        export::gen_ppm(
            &mut img,
            format!("/home/max/results/run7/raytraced_image_1.png").to_string(),
        );
        //std_div_renderer.render(&mut img);
        //export::gen_ppm(&mut img, "/home/max/raytraced_image2.png".to_string());
    });
}
