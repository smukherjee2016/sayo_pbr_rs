use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
mod utilities;
mod common;
use crate::common::*;

use toml::Value;

#[derive(Debug, Default)]
pub  struct SceneConfig {
    pub scene_file_name: PathBuf,
    pub out_file : PathBuf,
    pub image : Vec<Color>
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

    pub fn parse_scene(&mut self) -> Result<(), Box<dyn Error>> {

        let scene_file_contents = fs::read_to_string(&self.scene_file_name)?;

        let parsed_scene_toml = scene_file_contents.parse::<Value>().unwrap();

        println!("{:?}", parsed_scene_toml);

        let output_file_name = &parsed_scene_toml["renderer"]["hdr_output_file"].as_str().unwrap().to_string();
        let output_file_full_path = "sandbox/".to_string() + output_file_name;
        println!("{:?}", output_file_full_path);
        self.out_file = PathBuf::from(output_file_full_path);
        //Return nothing if all okay, return error otherwise
        Ok(())
    }

    pub fn write_output(&self) {
        utilities::imageutils::write_pfm(self.out_file.clone(), self.image.clone());
    }
}

