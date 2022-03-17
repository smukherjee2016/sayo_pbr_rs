use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
use crate::SceneCamera;
use ndarray::ArrayViewMut2;
use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::Mutex;
use tev_client::PacketUpdateImage;
use tev_client::TevClient;

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
        tev_client: Arc<Mutex<TevClient>>,
    ) {
        let film = film.as_ref().borrow();
        let mut pixel_values_for_viewer: Vec<f32> = vec![];
        // Tile ID is issued in L -> R and then T -> B order, starting with 0 from top left corner
        let x_offset: i32 = tile_id / (film.width / 16);
        let y_offset: i32 = tile_id % (film.width / 16);

        let x_starting_offset: i32 = y_offset * 16;
        let y_starting_offset: i32 = x_offset * 16;

        for x_local in 0..16 {
            for y_local in 0..16 {
                let y = x_offset * 16 + x_local as i32;
                let x = y_offset * 16 + y_local as i32;

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
                pixel_values_for_viewer.push(pixel_value.x as f32);
                pixel_values_for_viewer.push(pixel_value.y as f32);
                pixel_values_for_viewer.push(pixel_value.z as f32);

                curr_tile[[x_local as usize, y_local as usize]] = pixel_value;
            }
        }
        // Write tile here to framebuffer for viewer if any
        {
            let mut tev_client_mutable = tev_client.lock().unwrap();
            tev_client_mutable
                .send(PacketUpdateImage {
                    image_name: "test",
                    grab_focus: false,
                    channel_names: &["R", "G", "B"],
                    channel_offsets: &[0, 1, 2],
                    channel_strides: &[3, 3, 3],
                    x: x_starting_offset as u32,
                    y: y_starting_offset as u32,
                    width: 16,
                    height: 16,
                    data: &pixel_values_for_viewer,
                })
                .unwrap();
            //info!("Tile id: {:?}", tile_id);
            //let ten_millis = time::Duration::from_millis(10);

            //thread::sleep(ten_millis);
        }
        //info!("{:?}", curr_tile);
    }
}
