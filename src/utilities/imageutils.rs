use std::path::PathBuf;

use crate::common::*;
use byteorder::{ByteOrder, LittleEndian};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Default)]
struct PfmInfo {
    image: Vec<Spectrum>,
    width: i32,
    height: i32,
}

#[derive(Debug, Default)]
struct PfmPixel {
    r: f32,
    g: f32,
    b: f32,
}

pub fn write_pfm(
    file_path: PathBuf,
    pixels: Vec<Spectrum>,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(pixels.len(), (width * height) as usize);

    let display = file_path.display();
    //Create directory if does not exist
    std::fs::create_dir_all(PathBuf::from(file_path.parent().unwrap()))?;
    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    let mut buffer = BufWriter::new(file);
    //PFM Header
    writeln!(buffer, "PF")?;
    writeln!(buffer, "{} {}", width, height)?;
    writeln!(buffer, "{}", -1.0 as f32)?;

    for i in 0..(height) {
        for j in 0..(width) {
            let pixelvalue: &Spectrum = &pixels[(i * width + j) as usize];
            let r = pixelvalue.x as f32;
            let g = pixelvalue.y as f32;
            let b = pixelvalue.z as f32;

            //https://docs.rs/byteorder/1.3.1/byteorder/trait.ByteOrder.html#method.write_f32
            //x86_64 is LittleEndian
            let mut buf = [0; 4];
            LittleEndian::write_f32(&mut buf, r);
            buffer.write_all(&buf)?;
            LittleEndian::write_f32(&mut buf, g);
            buffer.write_all(&buf)?;
            LittleEndian::write_f32(&mut buf, b);
            buffer.write_all(&buf)?;
        }
    }

    Ok(())
}
