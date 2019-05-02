use sayo_pbr_rs::{SceneConfig, run};
use std::process;

fn main() {
    let mut args : Vec<String> = std::env::args().collect();

    let current_dir = std::env::current_dir().unwrap();
    println!("Current path: {:?}", std::fs::canonicalize(current_dir));

    //current_dir() is the root directory of the project, setting relative paths
    let scene_file_path = "scenes/default_scene.toml".to_string();
    args.push(scene_file_path);
    println!("{:?}", args);
    println!("Executable path is: {:?}", std::env::current_exe());
    let config = SceneConfig::parse_args(&args).unwrap_or_else(
      |err| {
          eprintln!("Problem parsing scene file: {}", err);
          process::exit(1);
      }
    );

    if let Err(e) = run(config) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}
