// This software is in the public domain.

use clap::Parser;
use img2tempdecal::*;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser, short, help = "Use point resampling")]
    point_resample: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(io::stdin())
	.expect("Can't obtain gif info.");

    decoder.next_frame_info()
	.expect("Can't obtain next frame info.");

    let mut img = vec![0; decoder.buffer_size()];
    decoder.read_into_buffer(&mut img)
	.expect("Can't obtain image data.");

    let width = decoder.width() as usize;
    let height = decoder.height() as usize;

    let wad = convert_texture_to_tempdecal(&img, width, height, cli.point_resample);

    let _ = io::stdout().write_all(&wad);
}
