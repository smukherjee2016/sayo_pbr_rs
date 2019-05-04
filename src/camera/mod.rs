pub mod pinholecamera;
use crate::common::*;
use crate::film::Film;
use std::rc::Rc;
use std::fmt::{Debug, Formatter, Error};

pub trait Camera {
    fn generate_camera_ray(&mut self, x : i32, y : i32, film : Rc<Film>) -> Ray;
}

impl Debug for Box<Camera> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        unimplemented!() //Base class so hope we won't need to print this
    }
}

impl Default for Box<Camera> {
    fn default() -> Self {
        unimplemented!()
    }
}