use std::path::PathBuf;

use crate::common::*;
use std::fs::File;
use std::io::{prelude, BufWriter, Write};
use std::error::Error;
use byteorder::{ByteOrder, LittleEndian, BigEndian};


#[derive(Debug, Default)]
struct PfmInfo {
    image : Vec<Color>,
    width : i32,
    height : i32

}

#[derive(Debug, Default)]
struct PfmPixel {
    r : f32,
    g: f32,
    b: f32
}

pub fn write_pfm(filepath : PathBuf, pixels : Vec<Color>, width : i32, height : i32) -> Result<(), String> {

    assert_eq!(pixels.len(), (width * height) as usize);

    let display = filepath.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&filepath) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    let mut buffer = BufWriter::new(file);
    //PFM Header
    writeln!(buffer, "PF");
    writeln!(buffer, "{} {}", width, height);
    writeln!(buffer, "{}", -1.0 as f32);


    for i in 0..(height) {
        for j in 0..(width) {
            let pixelvalue : Color = pixels[(i*width+j) as usize];
            let r  = (pixelvalue.x as f32);
            let g  = (pixelvalue.y as f32);
            let b  = (pixelvalue.z as f32);

            //https://docs.rs/byteorder/1.3.1/byteorder/trait.ByteOrder.html#method.write_f32
            //x86_64 is LittleEndian
            let mut buf = [0; 4];
            LittleEndian::write_f32(&mut buf, r);
            buffer.write(&buf);
            LittleEndian::write_f32(&mut buf, g);
            buffer.write(&buf);
            LittleEndian::write_f32(&mut buf, b);
            buffer.write(&buf);

        }
    }


    Ok(())
}