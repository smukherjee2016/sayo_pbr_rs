use crate::common::*;
use crate::integrators::*;
use crate::SceneConfig;

pub struct TestIntegrator;

impl Integrator for TestIntegrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32) {
        let camera = &mut scene.camera;
        let mut film = &mut scene.film;

        for i in 0..(film.height * film.width) {
            let position_in_film = i;
            let x = position_in_film % film.width;
            let y = position_in_film / film.width;

            let mut pixel_value: Spectrum = ZERO_VEC3;
            for _j in 0..samples_count {
                for _k in 0..bounces_count {
                    //Core Integrator code goes here
                    let ray = camera.generate_camera_ray(x, y, &mut film);
                    //info!("Ray info: {:?}", &ray);
                    let x;
                    let y;
                    let z;
                    if ray.d.x > 0.0 {
                        x = ray.d.x;
                    } else {
                        x = -ray.d.x;
                    }
                    if ray.d.y > 0.0 {
                        y = ray.d.y;
                    } else {
                        y = -ray.d.y;
                    }
                    if ray.d.z > 0.0 {
                        z = ray.d.z;
                    } else {
                        z = -ray.d.z;
                    }

                    pixel_value = Spectrum { x, y, z };
                }
            }
            film.image[position_in_film as usize] = pixel_value.normalize();
            //info!("{:?}",pixel_value);
            if (position_in_film as f32 / (film.height * film.width) as f32) * 100.0 % 1.0 == 0.0 {
                info!(
                    "Finished {} percent.",
                    ((position_in_film as f32 / (film.height * film.width) as f32) * 100.0) as i32
                );
            }
        }
        info!("Finished running render()");
    }
}
