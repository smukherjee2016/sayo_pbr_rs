use sayo_pbr_rs::{SceneConfig};
use std::process;



fn main() {
    let mut args : Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    println!("Current path: {:?}", std::fs::canonicalize(current_dir).unwrap());

    //current_dir() is the root directory of the project, setting relative paths
    //If no arguments specified, try to use a default scene
    if args.len() == 1 {
        let scene_file_path = "scenes/default_scene.toml".to_string();
        args.push(scene_file_path);
    }
    println!("{:?}", args);
    let mut scene_config = SceneConfig::default();

    let _is_parse_okay = scene_config.parse_args(&args).unwrap_or_else(
      |err| {
          eprintln!("Problem parsing scene file: {}", err);
          process::exit(1);
      }
    );

    if let Err(e) = scene_config.parse_scene() {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }

    scene_config.write_output();
}
