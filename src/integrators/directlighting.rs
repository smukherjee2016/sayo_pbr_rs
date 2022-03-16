use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
use crate::SceneCamera;
use ndarray::ArrayViewMut2;
use std::borrow::Borrow;
use std::sync::Arc;

pub struct DirectLightingIntegrator;

impl DirectLightingIntegrator {
    #[allow(clippy::too_many_arguments)]
    pub fn integrate(
        mut curr_tile: ArrayViewMut2<'_, Spectrum>,
        tile_id: i32,
        samples_count: u32,
        bounces_count: u32,
        camera: Arc<SceneCamera>,
        geometries: Arc<dyn Boundable>,
        //geometries: Arc<SceneGeometries>,
        film: Arc<Film>,
        t_min: fp,
        t_max: fp,
    ) {
        let film = film.as_ref().borrow();
        // TODO: Fix the indexing using real-time viewer assistance; this gives correct image but unclear why
        let y_offset: i32 = tile_id % (film.width / 16);
        let x_offset: i32 = tile_id / (film.width / 16);
        for x_local in 0..16 {
            for y_local in 0..16 {
                let x = x_offset * 16 + x_local as i32;
                let y = y_offset * 16 + y_local as i32;

                let mut pixel_value: Spectrum = Spectrum::default();
                for _j in 0..samples_count {
                    for _k in 0..bounces_count {
                        //Core Integrator code goes here
                        let ray = camera.generate_camera_ray(x, y, film);
                        //info!("Ray info: {:?}", &ray);
                        let intersection = geometries.check_intersection_and_return_closest_hit(
                            ray.clone(),
                            t_min,
                            t_max,
                        );
                        match intersection {
                            Some(intersection_info) => {
                                pixel_value += intersection_info.normal;
                                //info!("{:?}", pixel_value);
                            }
                            None => {
                                pixel_value += Vector3::new(0.5, 0.5, 0.5);
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
                curr_tile[[x_local as usize, y_local as usize]] = pixel_value;
            }
        }
        // Write tile here to framebuffer for viewer if any
        //info!("{:?}", curr_tile);
    }
}
