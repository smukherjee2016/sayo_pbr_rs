use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;

#[macro_use]
extern crate toml;

use toml::Value;

#[derive(Debug, Default)]
pub  struct SceneConfig {
    pub scene_file_name: PathBuf,
    pub out_file_name : PathBuf,

}

impl SceneConfig {
    pub fn parse_args(&mut self, args: &[String]) -> Result<(), String> {
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

        self.scene_file_name = scene_filename;

        Ok(())

    }

    pub fn parse_scene(&self) -> Result<(), Box<dyn Error>> {

        let scene_file_contents = fs::read_to_string(&self.scene_file_name)?;


        let value = scene_file_contents.parse::<Value>().unwrap();

        println!("{:?}", value);
        //Return nothing if all okay, return error otherwise
        Ok(())
    }

}

