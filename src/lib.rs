use std::error::Error;
use std::path::{Path, PathBuf};

pub struct SceneConfig {
    pub scene_file_name: PathBuf
}

impl SceneConfig {
    pub fn parse_args(args: &[String]) -> Result<SceneConfig, String> {
        let mut scene_filename = PathBuf::from("");
        for arg in args {
            if arg.contains(".toml"){
                scene_filename = Path::new(arg).canonicalize().unwrap_or_else( |err| {
                    eprintln!("Error for scene file: {} : {:?}", arg, err.to_string());
                    PathBuf::from("")
                    }
                );
            }
        }

        //Error cases
        if scene_filename.to_str() == Some("") {
            return Err("No scene configuration file specified or something else went wrong!".to_string());
        }

        Ok(SceneConfig{ scene_file_name: scene_filename })

    }
}

pub fn run(config : SceneConfig) -> Result<(), Box<dyn Error>> {


    //Return nothing if all okay, return error otherwise
    Ok(())
}