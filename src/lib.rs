//!
#![warn(rust_2018_idioms)]
use log::{info, warn};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
pub mod accel;
mod camera;
pub mod common;
mod film;
mod geometry;
pub mod integrators;
mod utilities;

use crate::accel::aabb::Boundable;
use crate::camera::pinholecamera::PinholeCamera;
use crate::camera::Camera;
use crate::common::*;
use crate::film::Film;
use crate::geometry::triangle::{Triangle, TriangleMesh};
use crate::integrators::baseintegrator::Integrators;
use std::sync::Arc;
use toml::Value;

pub struct SceneConfig {
    pub integrator: Integrators,
}

pub struct FileNames {
    pub scene_file_name: PathBuf,
    pub out_file: PathBuf,
}

#[derive(Default, Clone)]
pub struct Tile {
    pub start_index: i32,
    pub num_pixels: usize,
    pub pixels: Vec<Spectrum>,
}

pub struct SceneGeometries {
    pub geometries: Vec<Arc<dyn Boundable>>,
}

pub struct SceneCamera {
    pub camera: Box<dyn Camera + Send + Sync>,
}

#[derive(Default)]
pub struct ImageBuffer {
    image: Vec<Spectrum>,
}

impl SceneConfig {
    pub fn parse_args(args: &[String]) -> (PathBuf, toml::Value) {
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
        let scene_file_contents = fs::read_to_string(scene_filename.clone()).unwrap();

        let parsed_scene_result = scene_file_contents.parse::<Value>();

        //info!(&parsed_scene_toml);
        match parsed_scene_result {
            Ok(parsed_scene_toml) => (scene_filename, parsed_scene_toml),
            Err(e) => {
                panic!("Failed to parse scene file with error: {:?}", e);
            }
        }
    }

    pub fn construct_film(parsed_scene_toml: toml::Value) -> Film {
        //Film
        let width = parsed_scene_toml["camera"]["resolution"][0]
            .as_float()
            .unwrap() as i32;
        let height = parsed_scene_toml["camera"]["resolution"][1]
            .as_float()
            .unwrap() as i32;
        let fov_degrees = parsed_scene_toml["camera"]["fov"].as_float().unwrap() as fp;
        let mut film = Film::default();
        film.new_film(width, height, fov_degrees);
        film
    }

    pub fn construct_scene(
        scene_filename: PathBuf,
        parsed_scene_toml: toml::Value,
    ) -> Result<(SceneConfig, FileNames), Box<dyn Error>> {
        //Material

        //Integrator
        let type_of_integrator: Integrators;
        let integrator_string = &parsed_scene_toml["integrator"]["type"]
            .as_str()
            .unwrap()
            .to_ascii_lowercase();
        match integrator_string.as_ref() {
            "direct_lighting" => {
                type_of_integrator = Integrators::DirectLighting;
            }

            "path_tracer_bsdf" => {
                type_of_integrator = Integrators::PathTracerBSDF;
            }

            "path_tracer_nee" => {
                type_of_integrator = Integrators::PathTracerNEE;
            }

            _ => {
                warn!(
                    "Warning: Found unsupported integrator {}, falling back to DirectLighting...",
                    integrator_string
                );
                type_of_integrator = Integrators::DirectLighting;
            }
        }

        //Output pfm
        let output_file_name = &parsed_scene_toml["renderer"]["hdr_output_file"]
            .as_str()
            .unwrap()
            .to_string();
        let output_file_full_path = "sandbox/".to_string() + output_file_name;
        info!("{}", &output_file_full_path);
        let out_file = PathBuf::from(output_file_full_path);

        Ok((
            SceneConfig {
                integrator: type_of_integrator,
            },
            FileNames {
                scene_file_name: scene_filename,
                out_file,
            },
        ))
    }
}

impl SceneCamera {
    pub fn construct_camera(parsed_scene_toml: toml::Value) -> SceneCamera {
        //Camera
        let camera_position: Point3 = Point3 {
            x: parsed_scene_toml["camera"]["transform"]["position"][0]
                .as_float()
                .unwrap() as fp,
            y: parsed_scene_toml["camera"]["transform"]["position"][1]
                .as_float()
                .unwrap() as fp,
            z: parsed_scene_toml["camera"]["transform"]["position"][2]
                .as_float()
                .unwrap() as fp,
        };

        let camera_look_at: Point3 = Point3 {
            x: parsed_scene_toml["camera"]["transform"]["look_at"][0]
                .as_float()
                .unwrap() as fp,
            y: parsed_scene_toml["camera"]["transform"]["look_at"][1]
                .as_float()
                .unwrap() as fp,
            z: parsed_scene_toml["camera"]["transform"]["look_at"][2]
                .as_float()
                .unwrap() as fp,
        };

        let camera_up: Point3 = Point3 {
            x: parsed_scene_toml["camera"]["transform"]["up"][0]
                .as_float()
                .unwrap() as fp,
            y: parsed_scene_toml["camera"]["transform"]["up"][1]
                .as_float()
                .unwrap() as fp,
            z: parsed_scene_toml["camera"]["transform"]["up"][2]
                .as_float()
                .unwrap() as fp,
        };

        let type_of_camera = &parsed_scene_toml["camera"]["type"].as_str().unwrap();
        let camera: Box<dyn Camera + Send + Sync>;
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
        SceneCamera { camera }
    }

    pub fn generate_camera_ray(&self, x: i32, y: i32, film: &Film) -> Ray {
        self.camera.generate_camera_ray(x, y, film)
    }
}

impl SceneGeometries {
    pub fn check_intersection_return_closest_hit(
        &self,
        ray: Ray,
        t_min: fp,
        t_max: fp,
    ) -> Option<IntersectionInfo> {
        let mut closest_intersection_info: IntersectionInfo = Default::default();
        let mut t_max = std::f64::INFINITY;
        let mut hit_something: bool = false;

        for geometry in &self.geometries {
            if let Some(intersection_info) =
                geometry.check_intersection_and_return_closest_hit(ray.clone(), t_min, t_max)
            {
                if intersection_info.t_intersection < t_max {
                    hit_something = true;
                    t_max = intersection_info.t_intersection;
                    closest_intersection_info = intersection_info;
                }
            }
        }
        if hit_something {
            Some(closest_intersection_info)
        } else {
            None
        }
    }

    pub fn construct_geometries(
        scene_filename: PathBuf,
        parsed_scene_toml: toml::Value,
    ) -> SceneGeometries {
        //Geometry
        let mut geometries: Vec<Arc<dyn Boundable>> = vec![];
        let mut num_triangles: usize = 0;
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
                        let mesh_absolute_path = current_directory.canonicalize().unwrap();
                        //info!(mesh_absolute_path);
                        let input_meshes = TriangleMesh::new(mesh_absolute_path);
                        for input_mesh in input_meshes {
                            let triangles: Vec<Triangle> = input_mesh.get_triangles_from_mesh();
                            for triangle in triangles {
                                geometries.push(Arc::new(triangle));
                            }
                        }
                        num_triangles += geometries.len()
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
        warn!("Total no. of triangles: {}", num_triangles);
        SceneGeometries { geometries }
    }
}

impl ImageBuffer {
    pub fn new(size: usize) -> ImageBuffer {
        ImageBuffer {
            image: vec![Vector3::from(0.5); size],
        }
    }

    pub fn write_tile(&mut self, tile: Tile) {
        let starting_index = tile.start_index as usize;
        let num_pixels_to_write = tile.num_pixels;
        self.image[starting_index..(starting_index + num_pixels_to_write)]
            .clone_from_slice(&tile.pixels[0..num_pixels_to_write]);
    }
}

pub fn write_output(
    out_file: PathBuf,
    film: Film,
    image_buffer: ImageBuffer,
) -> Result<(), Box<dyn Error>> {
    let borrowed_film = film;
    let width = borrowed_film.width;
    let height = borrowed_film.height;

    utilities::imageutils::write_pfm(out_file, image_buffer.image, width, height)
}
