use clap::Parser;
use img2tempdecal::*;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    input: Option<PathBuf>,

    #[clap(value_parser, short, help = "Output file name")]
    output: Option<PathBuf>,

    #[clap(value_parser, short, help = "Larger size tempdecal for Sven Co-op")]
    large: bool,

    #[clap(value_parser, short, help = "Use point resampling")]
    point_resample: bool,
}

fn main() {
    let cli = Cli::parse();

    let (img, w, h) = if let Some(i) = cli.input {
        let file = File::open(i);
        let file = file.expect("Failed to open input file.");
        get_image(file)
    } else {
        get_image(io::stdin())
    };

    if let Some(o) = cli.output {
        let file = File::open(o);
        let mut file = file.expect("Failed to open input file.");
        convert_texture_to_tempdecal(&img, w, h, cli.large, cli.point_resample, &mut file)
    } else {
        convert_texture_to_tempdecal(&img, w, h, cli.large, cli.point_resample, &mut io::stdout())
    }
    .expect("Error occured while writing tempdecal.");
}

fn get_image<T: io::Read>(input: T) -> (Vec<u8>, usize, usize) {
    let mut png_reader = png::Decoder::new(input)
        .read_info()
        .expect("Can't obtain png info.");

    if png_reader.output_color_type().0 != png::ColorType::Rgba {
        panic!("Input must be RGBA png file.");
    }

    let mut img = vec![0; png_reader.output_buffer_size()];
    png_reader
        .next_frame(&mut img)
        .expect("Can't obtain image data.");

    let info = png_reader.info();
    (img, info.width as usize, info.height as usize)
}
