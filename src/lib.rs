use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
mod camera;
mod common;
mod film;
mod geometry;
pub mod integrators;
mod utilities;

use crate::camera::pinholecamera::PinholeCamera;
use crate::camera::Camera;
use crate::common::*;
use crate::film::Film;
use crate::geometry::triangle::{Triangle, TriangleMesh};
use crate::geometry::Hitable;
use std::cell::RefCell;
use toml::Value;

pub struct SceneConfig {
    pub scene_file_name: PathBuf,
    pub out_file: PathBuf,
    pub film: RefCell<Film>,
    pub camera: Box<dyn Camera>,
    pub geometries: Vec<Box<dyn Hitable>>,
}

impl SceneConfig {
    pub fn parse_args_and_construct_scene(args: &[String]) -> Result<SceneConfig, Box<dyn Error>> {
        let mut scene_filename = PathBuf::from("");
        for arg in args {
            if arg.contains(".toml") {
                scene_filename = PathBuf::from(Path::new(arg));
            }
        }

        //Error cases
        if scene_filename.to_str() == Some("") {
            panic!(
                "No scene configuration file specified or something else went wrong!".to_string()
            );
        }

        //Parse scene
        let scene_file_contents = fs::read_to_string(scene_filename.clone())?;

        let parsed_scene_toml = scene_file_contents.parse::<Value>().unwrap();

        //dbg!(&parsed_scene_toml);

        //Film
        let width = *(&parsed_scene_toml["camera"]["resolution"][0]
            .as_float()
            .unwrap()) as i32;
        let height = *(&parsed_scene_toml["camera"]["resolution"][1]
            .as_float()
            .unwrap()) as i32;
        let fov_degrees = *(&parsed_scene_toml["camera"]["fov"].as_float().unwrap()) as fp;
        let mut film = Film::default();
        film.new(width, height, fov_degrees);

        //Camera
        let camera_position: Point3 = Point3 {
            x: *(&parsed_scene_toml["camera"]["transform"]["position"][0]
                .as_float()
                .unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["position"][1]
                .as_float()
                .unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["position"][2]
                .as_float()
                .unwrap()) as fp,
        };

        let camera_look_at: Point3 = Point3 {
            x: *(&parsed_scene_toml["camera"]["transform"]["look_at"][0]
                .as_float()
                .unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["look_at"][1]
                .as_float()
                .unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["look_at"][2]
                .as_float()
                .unwrap()) as fp,
        };

        let camera_up: Point3 = Point3 {
            x: *(&parsed_scene_toml["camera"]["transform"]["up"][0]
                .as_float()
                .unwrap()) as fp,
            y: *(&parsed_scene_toml["camera"]["transform"]["up"][1]
                .as_float()
                .unwrap()) as fp,
            z: *(&parsed_scene_toml["camera"]["transform"]["up"][2]
                .as_float()
                .unwrap()) as fp,
        };

        let type_of_camera = &parsed_scene_toml["camera"]["type"].as_str().unwrap();
        let camera: Box<dyn Camera>;
        match *type_of_camera {
            "pinhole" => {
                camera = Box::new(PinholeCamera::new(
                    camera_position,
                    camera_look_at,
                    camera_up,
                ));
            }
            _ => {
                warn!("Warning: unknown or unsupported camera type, trying to fall back to pinhole camera");
                camera = Box::new(PinholeCamera::new(
                    camera_position,
                    camera_look_at,
                    camera_up,
                ));
            }
        }

        //Geometry
        let mut geometries: Vec<Box<dyn Hitable>> = vec![];

        for i in &parsed_scene_toml["primitives"].as_array() {
            for j in *i {
                let type_of_geometry = j["type"].as_str().unwrap();
                //Triangle mesh
                match type_of_geometry {
                    "mesh" => {
                        //Process the file path to ensure the meshes are found
                        let mut current_directory = PathBuf::from(scene_filename.parent().unwrap());
                        let mesh_location_and_name = j["file"].as_str().unwrap();
                        current_directory.push(mesh_location_and_name);
                        let mesh_absolute_path = current_directory.canonicalize()?;
                        //dbg!(mesh_absolute_path);
                        let input_meshes = TriangleMesh::new(mesh_absolute_path);
                        for input_mesh in input_meshes {
                            let triangles: Vec<Triangle> = input_mesh.get_triangles_from_mesh();
                            for triangle in triangles {
                                geometries.push(Box::new(triangle));
                            }
                        }
                    }
                    _ => {
                        warn!(
                            "Warning: found unsupported geometry type {}, skipping...",
                            type_of_geometry
                        );
                    }
                }
            }
        }

        //Material

        //Output pfm
        let output_file_name = &parsed_scene_toml["renderer"]["hdr_output_file"]
            .as_str()
            .unwrap()
            .to_string();
        let output_file_full_path = "sandbox/".to_string() + output_file_name;
        dbg!(&output_file_full_path);
        let out_file = PathBuf::from(output_file_full_path);

        Ok(SceneConfig {
            scene_file_name: scene_filename,
            out_file: out_file,
            film: RefCell::new(film),
            camera: camera,
            geometries: geometries,
        })
    }

    pub fn check_intersection_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let mut closest_intersection_info: IntersectionInfo = Default::default();
        let mut t_max = std::f64::INFINITY;
        let mut hit_something: bool = false;

        for geometry in &self.geometries {
            match geometry.check_intersection_and_return_closest_hit(ray.clone()) {
                Some(intersection_info) => {
                    if intersection_info.t_intersection < t_max {
                        hit_something = true;
                        t_max = intersection_info.t_intersection;
                        closest_intersection_info = intersection_info;
                    }
                }
                None => {}
            }
        }
        if hit_something == true {
            Some(closest_intersection_info)
        }
        else { None }
    }

    pub fn write_output(&self) -> Result<(), Box<dyn Error>> {
        let borrowed_film = self.film.borrow();
        let image = borrowed_film.image.clone();
        let width = borrowed_film.width;
        let height = borrowed_film.height;

        utilities::imageutils::write_pfm(self.out_file.clone(), image, width, height)
    }
}
