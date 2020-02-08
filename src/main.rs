use flexi_logger::{with_thread, Logger};
use log::warn;
use sayo_pbr_rs::integrators::baseintegrator::*;
use sayo_pbr_rs::integrators::Integrator;
use sayo_pbr_rs::{SceneCamera, SceneConfig, SceneGeometries};
use std::error::Error;
use std::time::Instant;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    Logger::with_env_or_str("info")
        .format(with_thread)
        .start()
        .unwrap();

    let mut args: Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    dbg!(std::fs::canonicalize(current_dir).unwrap());

    //current_dir() is the root directory of the project, setting relative paths
    //If no arguments specified, try to use a default scene
    if args.len() == 1 {
        let scene_file_path = "scenes/simple_cube/simple_cube_scene.toml".to_string();
        args.push(scene_file_path);
    }
    dbg!(&args);

    let start = Instant::now();
    let scene_config_tuple = SceneConfig::parse_args(&args);
    let scene_filename = scene_config_tuple.0;
    let parsed_scene_config = scene_config_tuple.1;
    let scene_config =
        SceneConfig::construct_scene(scene_filename.clone(), parsed_scene_config.clone()).unwrap();
    let scene_camera = SceneCamera::construct_camera(parsed_scene_config.clone());
    let scene_geometries =
        SceneGeometries::construct_geometries(scene_filename.clone(), parsed_scene_config.clone());

    let film = SceneConfig::construct_film(parsed_scene_config.clone());
    let tiles = BaseIntegrator::render(
        Arc::new(scene_config),
        1,
        1,
        Arc::new(scene_camera),
        Arc::new(scene_geometries),
        Arc::new(film.clone()),
    );

    for tile in tiles {
        //warn!("{}", tile.start_index);
        film.borrow_mut().write_tile(tile);
    }

    let duration = start.elapsed();
    warn!("Total time taken: {:?}", duration);

    scene_config.write_output(film.into_inner())?;

    Ok(())
}
