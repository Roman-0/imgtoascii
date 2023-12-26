use clap::Parser;
use image::open;
use image::{DynamicImage, Pixel};

fn main() {
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Path to image
        #[arg(short, long)]
        path: String,

        /// Width of the ASCII image
        #[arg(short, long, default_value_t = 32)]
        resolution: u32,
    }
    let args = Args::parse();

    let ascii_resolution_y: u32 = args.resolution * 2;
    let ascii_resolution_x: u32 = args.resolution;

    let luminance_scale_small: String = String::from(".,-~:;=!*#$@");
    let step_small: u8 = (255 / luminance_scale_small.len()).try_into().unwrap();

    let rgba = open(args.path).unwrap().into_rgba8();
    let gray = DynamicImage::ImageRgba8(rgba).into_luma8();

    let image_height = gray.height();
    let image_width = gray.width();
    let mut ascii_buf = String::new();

    for row in 0..ascii_resolution_x {
        for col in 0..ascii_resolution_y {
            let y = image_width / (ascii_resolution_x + 1) * (row + 1);
            let x = image_height / (ascii_resolution_y + 1) * (col + 1);

            //TAKE AVERAGE PIXEL VALUE
            let pixel = gray.get_pixel(x, y);
            let brightness = pixel.channels()[0];
            let character = luminance_scale_small
                .chars()
                .nth((brightness / step_small).into());
            ascii_buf.push(character.unwrap_or_else(|| {
                println!("{},{}", brightness, step_small);
                '.'
            }));
        }
        ascii_buf.push('\n');
    }

    println!("{}", ascii_buf);
}
