use flexi_logger::{with_thread, Logger};
use log::{info, warn};
use ndarray::Array2;
use sayo_pbr_rs::accel::bvh_node::BvhNode;
use sayo_pbr_rs::common::*;
use sayo_pbr_rs::integrators::baseintegrator::*;
use sayo_pbr_rs::integrators::Integrator;
use sayo_pbr_rs::{write_output, ImageBuffer, SceneCamera, SceneConfig, SceneGeometries};
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use tev_client::PacketCreateImage;
use tev_client::TevClient;

fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_env_or_str("info")?
        .format(with_thread)
        .start()
        .unwrap();

    let mut args: Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    info!("{:?}", std::fs::canonicalize(current_dir).unwrap());

    //current_dir() is the root directory of the project, setting relative paths
    //If no arguments specified, try to use a default scene
    if args.len() == 1 {
        //let scene_file_path = "scenes/dragon/dragon_scene.toml".to_string();
        args.push("--release".to_string());
        let scene_file_path = "scenes/teapot/teapot_test_scene.toml".to_string();
        //let scene_file_path = "scenes/simple_cube/simple_cube_scene.toml".to_string();
        args.push(scene_file_path);
    }
    info!("{:?}", &args);

    let mut tev_client = TevClient::spawn_path_default()?;

    let mut start = Instant::now();
    let scene_config_tuple = SceneConfig::parse_args(&args);
    let scene_filename = scene_config_tuple.0;
    let parsed_scene_config = scene_config_tuple.1;
    let (scene_config, file_names) =
        SceneConfig::construct_scene(scene_filename.clone(), parsed_scene_config.clone()).unwrap();
    let scene_camera = SceneCamera::construct_camera(parsed_scene_config.clone());
    let scene_geometries =
        SceneGeometries::construct_geometries(scene_filename, parsed_scene_config.clone());
    let film = SceneConfig::construct_film(parsed_scene_config);
    let duration_init = start.elapsed();
    warn!("Time to init scene: {:?}", duration_init);
    let root_bvh = BvhNode::construct_bvh(scene_geometries.geometries.clone(), 0);
    let duration_bvh = start.elapsed();
    warn!("Time to create BVH: {:?}", duration_bvh);

    // Initalize tev client
    tev_client.send(PacketCreateImage {
        image_name: "test",
        grab_focus: false,
        width: film.width as u32,
        height: film.height as u32,
        channel_names: &["R", "G", "B"],
    })?;

    start = Instant::now();
    let tiles: Array2<Spectrum> = BaseIntegrator::render(
        Arc::new(scene_config),
        1,
        1,
        Arc::new(scene_camera),
        root_bvh,
        //Arc::new(scene_geometries),
        Arc::new(film.clone()),
        1e-5,
        fp::MAX,
        Arc::new(Mutex::new(tev_client)),
    );

    let mut image_buffer = ImageBuffer::new((film.height * film.width) as usize);
    image_buffer.write_tile(tiles);

    let duration = start.elapsed();
    warn!("Total time taken: {:?}", duration);

    write_output(file_names.out_file, film, image_buffer)?;

    Ok(())
}
