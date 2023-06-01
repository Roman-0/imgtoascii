use image::{open};
use image::{DynamicImage,Pixel};
use clap::Parser;

fn main(){
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Path of Image
        #[arg(short, long)]
        path: String,

        /// Width of Ascii art in characters
        #[arg(short, long, default_value_t = 32)]
        resolution: u32,
    }
    let args = Args::parse();

    let ASCII_RESOLUTION_Y: u32 = args.resolution*2;
    let ASCII_RESOLUTION_X: u32 = args.resolution;

    let LUMINANCE_SCALE_SMALL: String = String::from(".,-~:;=!*#$@");
    let STEP_SMALL: u8 = (255/LUMINANCE_SCALE_SMALL.len()).try_into().unwrap();

    let rgba = open(args.path).unwrap().into_rgba8();
    let gray = DynamicImage::ImageRgba8(rgba).into_luma8();
    
    let image_height = gray.height();
    let image_width = gray.width();
    let mut ascii_buf = String::new();  

    for row in 0..ASCII_RESOLUTION_X {
        for col in 0..ASCII_RESOLUTION_Y{
            let y = image_width/(ASCII_RESOLUTION_X+1) * (row+1);
            let x = image_height/(ASCII_RESOLUTION_Y+1) * (col+1);
            let pixel = gray.get_pixel(x, y);
            let brightness = pixel.channels()[0]; 
            let character = LUMINANCE_SCALE_SMALL.chars().nth((brightness/STEP_SMALL).into());
            ascii_buf.push(character.unwrap_or_else(|| { 
                println!("{},{}",brightness,STEP_SMALL);
                '.'
            }));
        }
        ascii_buf.push('\n');
    }
    
    println!("{}",ascii_buf);
    gray.save("grey.png");
}
