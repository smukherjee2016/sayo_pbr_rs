use sayo_pbr_rs::SceneConfig;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    dbg!(std::fs::canonicalize(current_dir).unwrap());

    //current_dir() is the root directory of the project, setting relative paths
    //If no arguments specified, try to use a default scene
    if args.len() == 1 {
        let scene_file_path = "scenes/dragon/dragon_scene.toml".to_string();
        args.push(scene_file_path);
    }
    dbg!(&args);
    let scene_config = SceneConfig::parse_args_and_construct_scene(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing scene file: {}", err);
        process::exit(1);
    });

    scene_config.write_output()?;

    Ok(())
}
