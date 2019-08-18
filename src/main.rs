use flexi_logger::{with_thread, Logger};
use log::warn;
use sayo_pbr_rs::integrators::testintegrator::*;
use sayo_pbr_rs::integrators::Integrator;
use sayo_pbr_rs::SceneConfig;
use std::error::Error;
use std::process;
use std::time::Instant;

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
    let mut scene_config =
        SceneConfig::parse_args_and_construct_scene(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing scene file: {}", err);
            process::exit(1);
        });

    TestIntegrator::render(&mut scene_config, 1, 1);

    let duration = start.elapsed();
    warn!("Total time taken: {:?}", duration);

    scene_config.write_output()?;

    Ok(())
}
