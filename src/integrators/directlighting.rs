use crate::common::*;
use crate::integrators::baseintegrator::Tile;
use crate::SceneConfig;
use std::borrow::Borrow;

pub struct DirectLightingIntegrator;

impl DirectLightingIntegrator {
    pub fn integrate(
        scene: &SceneConfig,
        start_position_in_film: i32,
        samples_count: u32,
        bounces_count: u32,
    ) -> Tile {
        let mut tile: Tile = Tile {
            start_index: start_position_in_film,
            num_pixels: TILE_SIZE,
            pixels: vec![],
        };
        tile.pixels.resize(TILE_SIZE, Spectrum::from(0.0));

        for i in 0..tile.num_pixels {
            let film = scene.film.borrow();
            let x = (start_position_in_film + i as i32) % film.width;
            let y = (start_position_in_film + i as i32) / film.width;

            let mut pixel_value: Spectrum = Spectrum::default();
            for _j in 0..samples_count {
                for _k in 0..bounces_count {
                    //Core Integrator code goes here
                    let ray = scene.camera.generate_camera_ray(x, y, &film);
                    //info!("Ray info: {:?}", &ray);
                    let intersection = scene.check_intersection_return_closest_hit(ray.clone());
                    match intersection {
                        Some(intersection_info) => {
                            pixel_value += intersection_info.normal;
                            //info!("{:?}", pixel_value);
                        }
                        None => {
                            pixel_value += Vector3::new(0.0, 0.0, 0.0);
                        }
                    }
                }
            }
            pixel_value /= fp::from(samples_count);
            if !pixel_value.x.is_finite()
                || !pixel_value.y.is_finite()
                || !pixel_value.z.is_finite()
            {
                warn!(
                    "Value is infinite or NaN!! {} {} {} at pixel {} {}",
                    pixel_value.x, pixel_value.y, pixel_value.z, x, y
                );
            }
            tile.pixels[i] = pixel_value;
        }
        tile
    }
}
