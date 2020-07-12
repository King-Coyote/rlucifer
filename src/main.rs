extern crate raster;
extern crate rand;

use std::env;
use std::time::{Instant,};
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::runtime::Builder;
use basic_types::*;
use objects::*;
use image_conversion::convert_image;
use ray_tracing::trace_ray;
use std::sync::{Mutex, Arc, LockResult};
use std::process;
use futures::future::join_all;

mod basic_types;
mod objects;
mod image_conversion;
mod ray_tracing;
mod util;

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

    let mut rt = Builder::new()
        .threaded_scheduler()
        .build()
        .unwrap();

    let main_scene = Arc::new(temp_main_scene_setup());
    let camera = Arc::new(Camera::new(
        Vector3::new(0.55, 0.05, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        width as i32,
        height as i32
    ));
    
    // let mut pixels: Mutex<Vec<Vec<Pixel>>> = Mutex::new(vec![vec![Pixel::new_default(); height]; width]);
    let mut pixels: Arc<Mutex<Vec<Vec<Pixel>>>> = Arc::new(Mutex::new(vec![
        vec![Pixel::new_black(); width]; height
    ]));

    let time_before = Instant::now();
    let mut percent = Arc::new(Mutex::new(0.0));

    let mut joins = vec![];

    rt.block_on(async {
        for j in 0..height {
            let local_percent = percent.clone();
            let local_width = width.clone();
            let local_samples = samples.clone();
            let local_pixels = pixels.clone();
            let local_scene = main_scene.clone();
            let local_cam = camera.clone();
            joins.push(tokio::spawn(async move {
                let mut row: Vec<Pixel> = vec![];
                for i in 0..local_width {
                    let mut sum_radiance = Pixel::new_black();
                    for _ in 0..local_samples {   
                        let fsamples = local_samples as f32;
                        let scene_ray = Ray::new(local_cam.get_position(), local_cam.pixel_to_img(i, j));
                        if let Some(radiance) = trace_ray(&scene_ray, &local_scene, 0) {
                            sum_radiance = sum_radiance + (radiance / fsamples);
                        }
                    }
                    row.push(sum_radiance);
                    *local_percent.lock().unwrap() += 1.0;
                }
                local_pixels.lock().unwrap()[j] = row;
            }));
        }
        for join in joins.iter_mut() {
            let percent: f32 = (*percent.lock().unwrap() / (height * width) as f32) * 100.0;
            print!("Rendering percent complete: {}\r", percent);
            join.await.expect("thread panicked");
        }
    });

    let time_taken = Instant::now().duration_since(time_before).as_secs_f32();
    println!("Total time taken for render is: {} s", time_taken);

    convert_image(
        "render_img.png", width, height, 
        &pixels.lock().unwrap()
    ); 

}

fn temp_diffuse_plane(params: (&str, Vector3, Vector3, Pixel)) -> Box<Plane> {
    let mat = Material::new(params.3, Finish::Diffuse{roughness: 1.0, specularity: 0.0});
    Box::new(Plane::new(
        params.0.to_owned(), params.1, params.2, mat
    ))
}

fn temp_diffuse_sphere(params: (&str, Vector3, f32, Pixel)) -> Box<Sphere> {
    let mat = Material::new(params.3, Finish::Diffuse{roughness: 1.0, specularity: 0.0});
    Box::new(Sphere::new(
        params.0.to_owned(), params.1, params.2, mat
    ))
}

fn temp_main_scene_setup() -> Scene {
    let mut scene = Scene::new();

    let default_mat = Material::new(Pixel::new(0.5, 0.5, 0.5), Finish::Diffuse{roughness: 1.0, specularity: 0.0});

    let mut plane_params = vec![
        ("ceiling", Vector3::new(0.5, 0.5, 0.0), Vector3::new(0.0, 0.0, 1.0), Pixel::new(0.5, 0.5, 0.5)),
        ("left_wall", Vector3::new(0.0, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0), Pixel::new(0.0, 1.0, 0.0)),
        ("back_wall", Vector3::new(0.5, 1.0, 0.5), Vector3::new(0.0, -1.0, 0.0), Pixel::new(0.0, 0.0, 1.0)),
        ("right_wall", Vector3::new(1.0, 0.5, 0.5), Vector3::new(-1.0, 0.0, 0.0), Pixel::new(1.0, 0.0, 0.0)),
        ("floor", Vector3::new(0.5, 0.5, 1.0), Vector3::new(0.0, 0.0, -1.0), Pixel::new(1.0, 0.0, 1.0)),
        ("front_wall", Vector3::new(0.5, 0.0, 0.5), Vector3::new(0.0, 1.0, 0.0), Pixel::new(0.0, 1.0, 1.0)),
    ];

    let mut light_params = vec![
        (Vector3::new(0.5, 0.5, 0.05), 0.4, Pixel::new(1.0, 1.0, 1.0)),
    ];

    let mut sphere_params = vec![
        ("sphere1", Vector3::new(0.5, 0.9, 0.5), 0.1, Pixel::new(1.0, 1.0, 1.0)),
    ];

    for plane in plane_params.drain(..) {
        scene.objects.push(temp_diffuse_plane(plane));
    }

    for light in light_params.drain(..) {
        scene.lights.push(Light::new(
            light.0, light.1, light.2
        ));
    }

    for sphere in sphere_params.drain(..) {
        scene.objects.push(temp_diffuse_sphere(sphere));
    }

    return scene;

}