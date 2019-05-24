use crate::camera::Camera;
use crate::common::*;
use crate::film::Film;
use std::f32::INFINITY;

#[derive(Debug)]
pub struct PinholeCamera {
    origin: Point3,
    look_at: Point3, //look_at is a point, not direction
    up: Vec3,
    direction_to_look_at: Vec3,

    c_x: Vec3,
    c_y: Vec3,
    c_z: Vec3,
}

impl Default for PinholeCamera {
    fn default() -> Self {
        PinholeCamera {
            origin: ZERO_VEC3,
            look_at: ZERO_VEC3,
            up: ZERO_VEC3,
            direction_to_look_at: ZERO_VEC3,
            c_x: ZERO_VEC3,
            c_y: ZERO_VEC3,
            c_z: ZERO_VEC3,
        }
    }
}

fn make_basis_vectors(pinhole_camera: &mut PinholeCamera) {
    pinhole_camera.direction_to_look_at =
        cgmath::InnerSpace::normalize(pinhole_camera.look_at - pinhole_camera.origin);

    //Basis vectors at camera origin
    pinhole_camera.c_x = pinhole_camera
        .direction_to_look_at
        .cross(pinhole_camera.up)
        .normalize();
    pinhole_camera.c_y = pinhole_camera
        .c_x
        .cross(pinhole_camera.direction_to_look_at)
        .normalize();
    pinhole_camera.c_z = pinhole_camera.direction_to_look_at.normalize();
}

impl Camera for PinholeCamera {
    fn new(origin_: Point3, look_at: Point3, up_: Vec3) -> Self {
        let mut phc: PinholeCamera = PinholeCamera::default();

        phc.origin = origin_;
        phc.look_at = look_at;
        phc.up = up_;

        make_basis_vectors(&mut phc);

        phc
    }

    fn generate_camera_ray(&self, x: i32, y: i32, film: &Film) -> Ray {
        //Find point inside pixel coordinates
        let u: fp = (x as fp + 0.5) / film.width as fp;
        let v: fp = (y as fp + 0.5) / film.height as fp;

        //Find height and width of the image plane based on FOV, distance and aspect ratio
        //Use Y-FOV
        let height_image_plane: fp = 2.0 * film.distance_to_film * (film.fov / 2.0).tan();
        let width_image_plane: fp = height_image_plane * film.aspect_ratio;

        //Project u and v to image plane
        let x_image_plane = (u - 0.5) * width_image_plane;
        let y_image_plane = (v - 0.5) * height_image_plane;

        //Project to world space
        let position_pixel_in_image_space: Point3 = self.origin
            + film.distance_to_film * self.direction_to_look_at
            + x_image_plane * self.c_x
            + y_image_plane * self.c_y;

        let direction_in_image_space: Vec3 =
            (position_pixel_in_image_space - self.origin).normalize();

        Ray::new(
            self.origin,
            direction_in_image_space,
            EPSILON,
            INFINITY.into(),
        )
    }
}
