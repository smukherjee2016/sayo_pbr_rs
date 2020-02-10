use flexi_logger::{with_thread, Logger};
use log::{info, warn};
use sayo_pbr_rs::integrators::baseintegrator::*;
use sayo_pbr_rs::integrators::Integrator;
use sayo_pbr_rs::{write_output, ImageBuffer, SceneCamera, SceneConfig, SceneGeometries};
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    Logger::with_env_or_str("info")
        .format(with_thread)
        .start()
        .unwrap();

    let mut args: Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    info!(std::fs::canonicalize(current_dir).unwrap());

    //current_dir() is the root directory of the project, setting relative paths
    //If no arguments specified, try to use a default scene
    if args.len() == 1 {
        let scene_file_path = "scenes/dragon/dragon_scene.toml".to_string();
        args.push(scene_file_path);
    }
    info!(&args);

    let start = Instant::now();
    let scene_config_tuple = SceneConfig::parse_args(&args);
    let scene_filename = scene_config_tuple.0;
    let parsed_scene_config = scene_config_tuple.1;
    let (scene_config, file_names) =
        SceneConfig::construct_scene(scene_filename.clone(), parsed_scene_config.clone()).unwrap();
    let scene_camera = SceneCamera::construct_camera(parsed_scene_config.clone());
    let scene_geometries =
        SceneGeometries::construct_geometries(scene_filename, parsed_scene_config.clone());

    let film = SceneConfig::construct_film(parsed_scene_config);
    let tiles = BaseIntegrator::render(
        Arc::new(scene_config),
        1,
        1,
        Arc::new(scene_camera),
        Arc::new(scene_geometries),
        Arc::new(film.clone()),
    );

    let mut image_buffer = ImageBuffer::new((film.height * film.width) as usize);
    for tile in tiles {
        //warn!("{}", tile.start_index);
        image_buffer.write_tile(tile);
    }

    let duration = start.elapsed();
    warn!("Total time taken: {:?}", duration);

    write_output(file_names.out_file, film, image_buffer)?;

    Ok(())
}
