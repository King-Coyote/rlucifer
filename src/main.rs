extern crate raster;

use std::env;
use std::time::{Instant, Duration};
use basic_types::*;
use objects::*;
use image_conversion::convert_image;

mod basic_types;
mod objects;
mod image_conversion;

fn main() {

    let mut width: usize = 600;
    let mut height: usize = 480;
    let mut samples = 10;
    let args: Vec<String> = env::args().collect();
    
	if args.len() <= 2 {
		println!("Insufficent args specified, using default height=600/width=480/samples=10\n");
	} else {
        width = args[1].parse().unwrap();
        height = args[2].parse().unwrap();
        samples = args[3].parse().unwrap();
		if height <= 0 || width <= 0 {
            println!("Invalid width/height specified, exiting\n");
            return;
		}
		if samples == 0 {
			println!("Invalid samples specified, exiting\n");
			return;
		}
    }
    
    println!("Running with: width {}, height {}, samples {}.", width, height, samples);

    let mut mainScene = Scene::new();
    // let mut camera = Camera::new(
    //     Vector3::new(),
    //     Vector3::new(),
    //     width as i32,
    //     height as i32
    // );
    
    let mut pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::new_default(); height]; width];

    let time_before = Instant::now();
    
    for j in 0..height {
        print!("Rendering percent complete: {}\r", (j/height)*100);
        for i in 0..width {
            for s in 0..samples {   
                let mut radiance = Pixel::new_black();
                // let mut mainRay = Ray::new(camera.get_position(), camera.pixel_to_img(i, j));
                // raytracer.tracePixel(radiance, mainRay, mainScene, urb, 0);
                // pixels[i][j] += radiance / samples;
            }
        }
    }

    let time_taken = Instant::now().duration_since(time_before).as_secs_f32();
    println!("Total time taken for render is: {} s", time_taken);

    convert_image("render_img.png", width, height, &pixels); 

}
