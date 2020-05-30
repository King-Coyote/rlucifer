use std::env;
use basic_types::*;
use objects::*;

mod basic_types;
mod objects;

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

    // let mut mainScene = Scene::new();
	// // ObjectFactoryJson ofj("scene.json");
	// // mainScene.setObjects(ofj.createObjects());
    // // mainScene.setLights(ofj.createLights());
    
    // let mut pixels: Vec<Vec<Pixel>> = Vec::with_capacity(width);
	// for i in 0..width {
	// 	pixels[i] = Vec::with_capacity(height);
    // }
    
    // urb.setSeed((omp_get_thread_num() + 13)*47);
    // for j in 0..height {
    //     print!("Rendering percent complete: {}\r", (j/height)*100);
    //     for i in 0..width {
    //         for s in 0..samples {   
    //             let mut radiance = Pixel::new_blank();
    //             let mut mainRay = Ray::new(camera.getPosition(), camera.pixelToImage(i, j));
    //             raytracer.tracePixel(radiance, mainRay, mainScene, urb, 0);
    //             pixels[i][j] += radiance / samples;
    //         }
    //     }
    // }


}
