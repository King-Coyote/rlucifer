extern crate raster;
extern crate rand;

use std::env;
use std::time::{Instant, Duration};
use basic_types::*;
use objects::*;
use image_conversion::convert_image;
use ray_tracing::trace_ray;

mod basic_types;
mod objects;
mod image_conversion;
mod ray_tracing;
mod util;

fn main() {

    let mut width: usize = 600;
    let mut height: usize = 480;
    let mut samples = 1;
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

    let mut main_scene = temp_main_scene_setup();
    let camera = Camera::new(
        Vector3::new(0.55, 0.05, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        width as i32,
        height as i32
    );
    
    let mut pixels: Vec<Vec<Pixel>> = vec![vec![Pixel::new_default(); height]; width];

    let time_before = Instant::now();
    
    for j in 0..height {
        print!("Rendering percent complete: {}\r", (j/height)*100);
        for i in 0..width {
            for s in 0..samples {   
                let fsamples = samples as f32;
                let scene_ray = Ray::new(camera.get_position(), camera.pixel_to_img(i, j));
                let mut sum_radiance = Pixel::new_black();
                if let Some(radiance) = trace_ray(&scene_ray, &main_scene, 0) {
                    sum_radiance = sum_radiance + (radiance / fsamples);
                }
                pixels[i][j] = sum_radiance;
            }
        }
    }

    let time_taken = Instant::now().duration_since(time_before).as_secs_f32();
    println!("Total time taken for render is: {} s", time_taken);

    convert_image("render_img.png", width, height, &pixels); 

}

fn temp_diffuse_plane(params: (&str, Vector3, Vector3, Material)) -> Box<Plane> {
    Box::new(Plane::new(
        params.0.to_owned(), params.1, params.2, params.3
    ))
}

fn temp_main_scene_setup() -> Scene {
    let mut scene = Scene::new();

    let default_mat = Material::new(Pixel::new(0.5, 0.5, 0.5), Finish::Diffuse{roughness: 1.0, specularity: 0.0});

    let mut plane_params = vec![
        ("ceiling", Vector3::new(0.5, 0.5, 0.0), Vector3::new(0.0, 0.0, 1.0), default_mat.clone()),
        ("left_wall", Vector3::new(0.0, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0), default_mat.clone()),
        ("back_wall", Vector3::new(0.5, 1.0, 0.5), Vector3::new(0.0, -1.0, 0.0), default_mat.clone()),
        ("right_wall", Vector3::new(1.0, 0.5, 0.5), Vector3::new(-1.0, 0.0, 0.0), default_mat.clone()),
        ("floor", Vector3::new(0.5, 0.5, 1.0), Vector3::new(0.0, 0.0, -1.0), default_mat.clone()),
        ("front_wall", Vector3::new(0.5, 0.0, 0.5), Vector3::new(0.0, 1.0, 0.0), default_mat.clone()),
    ];

    let mut light_params = vec![
        (Vector3::new(0.5, 0.5, 0.05), 0.5, Pixel::new(1.0, 1.0, 1.0)),
    ];

    for plane in plane_params.drain(..) {
        scene.objects.push(temp_diffuse_plane(plane));
    }

    for light in light_params.drain(..) {
        scene.lights.push(Light::new(
            light.0, light.1, light.2
        ));
    }

    return scene;

    	// POSITIONING:
	// z=0 is the floor, 1 is the ceiling.
	//TODO create a data-driven way of constructing a scene
    // mainScene.addObject(new RenderObjectPlane(Vec(0.5, 0.5, 0), "Ceiling", Vec(0, 0, 1), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(0.9, 0.1, 0.2))));
	// mainScene.addObject(new RenderObjectPlane(Vec(0, 0.5, 0.5), "Left_Wall", Vec(1, 0, 0), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(1.0, 0, 0))));
	// mainScene.addObject(new RenderObjectPlane(Vec(0.5, 1, 0.5), "Back_Wall", Vec(0, -1, 0), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(1.0, 0, 0))));
	// mainScene.addObject(new RenderObjectPlane(Vec(1, 0.5, 0.5), "Right_Wall", Vec(-1, 0, 0), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(0.1, 1.0, 1.0))));
	// mainScene.addObject(new RenderObjectPlane(Vec(0.5, 0.5, 1), "Floor", Vec(0, 0, -1), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(1.0, 0, 0))));
	// mainScene.addObject(new RenderObjectPlane(Vec(0.5, 0, 0.5), "Front_Wall", Vec(0, 1, 0), RenderMaterial(RenderMaterialType::DIFFUSE, Pixel(1.0, 0, 0))));
}