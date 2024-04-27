use clap::Parser;
use image::open;
use image::Pixel;

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

    let ascii_resolution_y: u32 = args.resolution; //32
    let ascii_resolution_x: u32 = args.resolution*2; //32

    let luminance_scale: String = String::from(".,-~:;=!*#$@");
    let step_small: u8 = 21; //(255 / luminance_scale_small.len()).try_into().unwrap();

    let grey = open(args.path).unwrap_or_else(|err| {
        eprintln!("Error opening image. {}",err);
        std::process::exit(1);
    }).into_luma8();

    let image_height = grey.height(); //2353
    let image_width = grey.width(); //2000

    let mut ascii_buf = String::new();

    /*for row in 0..ascii_resolution_x{
        for col in 0..ascii_resolution_y{
            let y:u32 = (image_height / (ascii_resolution_y + 1) * (col + 1)).into();
            let x:u32 = (image_width / (ascii_resolution_x + 1) * (row + 1)).into();

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
    }*/
    let x_step: u32 = image_width / ascii_resolution_x;
    let y_step: u32 = image_height / ascii_resolution_y;
    for col in 1..ascii_resolution_y{
        for row in 1..ascii_resolution_x{
            let x: u32 = x_step * row;
            let y: u32 = y_step * col;

            let brightness = grey.get_pixel(x, y).channels()[0];
            let character = luminance_scale.chars().nth((brightness/step_small).into());

            ascii_buf.push(character.unwrap_or_else(|| {
                println!("{},{}", brightness, step_small);
                ' '
            }));
            
        }
        ascii_buf.push('\n');
    }
    println!("{}", ascii_buf);
}
/*
fn get_avg_pixel_brightness(x: u32, y: u32) -> u32 {

}
*/