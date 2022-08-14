use img2tempdecal::*;
use std::fs::File;
use std::{env, io};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} rgba.png > out", args[0]);
        return;
    }
    let file = File::open(&args[1]);
    let file = file.expect("Failed to open input file.");

    let png_reader = png::Decoder::new(file).read_info();
    let mut png_reader = png_reader.expect("Can't retrieve png info.");

    if png_reader.output_color_type().0 != png::ColorType::Rgba {
        panic!("Input must be RGBA png file.");
    }

    let mut img = vec![0; png_reader.output_buffer_size()];
    png_reader
        .next_frame(&mut img)
        .expect("Can't retrieve image data.");

    let info = png_reader.info();
    let r = convert_texture_to_tempdecal(
        &img,
        info.width as usize,
        info.height as usize,
        true,
        &mut io::stdout(),
    );
    r.expect("Error occured while writing.");
}
