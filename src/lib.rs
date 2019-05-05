use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
mod utilities;
mod common;
mod film;
mod camera;

use toml::Value;
use crate::film::Film;
use crate::camera::Camera;
use crate::common::*;
use crate::camera::pinholecamera::PinholeCamera;

pub struct SceneConfig {
    pub scene_file_name: PathBuf,
    pub out_file: PathBuf,
    pub film: Film,
    pub camera: Box < dyn Camera >,
}

impl SceneConfig {
    pub fn parse_args_and_construct_scene(args: &[String]) -> Result<SceneConfig, Box<dyn Error>> {

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
            panic!("No scene configuration file specified or something else went wrong!".to_string());
        }

        //Parse scene
        let scene_file_contents = fs::read_to_string(scene_filename.clone())?;

        let parsed_scene_toml = scene_file_contents.parse::<Value>().unwrap();

        //dbg!(&parsed_scene_toml);

        //Film
        let width = *(&parsed_scene_toml["camera"]["resolution"][0].as_float().unwrap()) as i32;
        let height = *(&parsed_scene_toml["camera"]["resolution"][1].as_float().unwrap()) as i32;
        let fov_degrees = *(&parsed_scene_toml["camera"]["fov"].as_float().unwrap()) as fp;
        let mut film = Film::default();
        film.new(width, height, fov_degrees);



        //Camera
        let camera_position : Point3 = Point3{
            x: *(&parsed_scene_toml["camera"]["transform"]["position"][0].as_float().unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["position"][1].as_float().unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["position"][2].as_float().unwrap()) as fp
        };

        let camera_look_at : Point3 = Point3{
            x: *(&parsed_scene_toml["camera"]["transform"]["look_at"][0].as_float().unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["look_at"][1].as_float().unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["look_at"][2].as_float().unwrap()) as fp
        };

        let camera_up : Point3 = Point3{
            x: *(&parsed_scene_toml["camera"]["transform"]["up"][0].as_float().unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["up"][1].as_float().unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["up"][2].as_float().unwrap()) as fp
        };

        let type_of_camera = &parsed_scene_toml["camera"]["type"].as_str().unwrap();
        let mut camera : Box<Camera> = Box::new(PinholeCamera::default());
        match *type_of_camera {
            "pinhole" => {
                camera = Box::new(PinholeCamera::new(camera_position, camera_look_at, camera_up));
            },
            _ => {
                eprintln!("Warning: unknown or unsupported camera type, trying to fall back to pinhole camera");
                camera = Box::new(PinholeCamera::new(camera_position, camera_look_at, camera_up));
            }
        }

        //Output pfm
        let output_file_name = &parsed_scene_toml["renderer"]["hdr_output_file"].as_str().unwrap().to_string();
        let output_file_full_path = "sandbox/".to_string() + output_file_name;
        dbg!(&output_file_full_path);
        let out_file = PathBuf::from(output_file_full_path);

        Ok(SceneConfig{
            scene_file_name: scene_filename,
            out_file: out_file,
            film: film,
            camera: camera
        })
    }

    pub fn write_output(&self) -> Result<(), Box<dyn Error>> {
        utilities::imageutils::write_pfm(self.out_file.clone(), self.film.image.clone(), self.film.width, self.film.height)
    }
}

