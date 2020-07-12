use crate::basic_types::Pixel;
use raster::{Image, Color};
use std::string::String;

pub fn convert_image(filename: &'static str, width: usize, height: usize, pixels: &[Vec<Pixel>]) 
{
    let mut img = Image::blank(width as i32, height as i32);
    raster::editor::fill(&mut img, Color::rgb(255, 0, 255)).unwrap();
    let size = pixels.len();
    let sizesize = pixels[0].len();
    for j in 0..height {
        for i in 0..width {
            img.set_pixel(i as i32, j as i32, Color::rgb(
                (pixels[j][i].r * 255.0) as u8,
                (pixels[j][i].g * 255.0) as u8,
                (pixels[j][i].b * 255.0) as u8
            )).unwrap();
        }
    }
    let mut full_path = String::new();
    full_path.push_str("/home/alex/projects/rlucifer/results/");
    full_path.push_str(filename);
    println!("Printing to: {}", full_path);
    raster::save(&img, &full_path).unwrap();
}