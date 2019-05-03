use std::path::PathBuf;

use crate::common::*;


#[derive(Debug, Default)]
struct PfmInfo {
    image : Vec<Color>,
    width : i32,
    height : i32

}

pub fn write_pfm(file : PathBuf, pixels : Vec<Color>) -> Result<(), String> {



    Ok(())
}