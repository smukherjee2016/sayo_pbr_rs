use crate::common::*;
use crate::SceneConfig;

pub struct DirectLightingIntegrator;

impl DirectLightingIntegrator {
    pub fn integrate(
        scene: &mut SceneConfig,
        position_in_film: i32,
        samples_count: u32,
        bounces_count: u32,
    ) {
        let film = &mut scene.film.borrow_mut();
        let x = position_in_film % film.width;
        let y = position_in_film / film.width;

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
        if !pixel_value.x.is_finite() || !pixel_value.y.is_finite() || !pixel_value.z.is_finite() {
            warn!(
                "Value is infinite or NaN!! {} {} {} at pixel {} {}",
                pixel_value.x, pixel_value.y, pixel_value.z, x, y
            );
        }
        film.image[position_in_film as usize] = pixel_value;
        //info!("{:?}",pixel_value);
        if (position_in_film as f32 / (film.height * film.width) as f32) * 100.0 % 10.0 == 0.0 {
            info!(
                "Finished {} percent.",
                ((position_in_film as f32 / (film.height * film.width) as f32) * 100.0) as i32
            );
        }
    }
}
