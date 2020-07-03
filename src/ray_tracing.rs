use crate::basic_types::*;
use crate::objects::*;
use rand::prelude::*;

const PI: f32 = 3.1415926;

pub fn trace_ray(main_ray: &Ray, scene: &Scene, depth: u32) -> Option<Pixel> {

    if depth > 5 {
        return None;
    }

    let intersect = scene.get_closest_intersect(main_ray)?;
    let (surface_radiance, ray) = interact_ray(&main_ray, &intersect);

    let explicit_light = get_explicit_light_contrib(&intersect, &scene);

    let mut radiance = Pixel::new_black();
    if let Some(li) = trace_ray(&mut main_ray, &scene, depth + 1) {
        radiance = radiance + (li * intersect.material.color);
    }
    radiance = radiance + explicit_light * intersect.material.color;
    
    Some(radiance)
}

fn interact_ray(ray: &Ray, hit: &Hit) -> (Pixel, Ray) {
    let mut rando = rand::thread_rng();
    let mut out_ray: Ray;
    let mut radiance =  Pixel::new_black();
    match hit.material.finish {
        Finish::Diffuse {specularity, roughness} => {
            // out_ray = sample_ray(&hit, roughness, &mut rando);
            // //FDG
            // let G = geometric_term(roughness);
            // let F = fresnel_term(roughness);
            // let D = normal_term(roughness);
            // let kd = 1.0 - F;
            // let denom = 4.0 * ray.dir.dot(&hit.normal) * out_ray.dir.dot(&hit.normal);
        },
        Finish::Mirror => {
            out_ray = test_sample_ray(&hit, &ray);
            radiance = hit.material.color.clone();
        }
        _ => {}
        // Diffuse {
        //     roughness: f32,
        //     specularity: f32,
        // },
        // Metallic {
        //     roughness: f32,
        //     specularity: f32,
        // },
        // Transparent {
        //     specularity: f32,
        //     refractive_index: f32,
        // },
        // Translucent,
        // Mirror,
    }
    (radiance, out_ray)
}

fn test_sample_ray(hit: &Hit, in_ray: &Ray) -> Ray {
    let cos_i = (hit.normal * -1.0).dot(&in_ray.dir);
    Ray::new(
        hit.point.clone(),
        (in_ray.dir + hit.normal * cos_i * 2.0).normalise()
    )
}

// fn sample_ray(hit: &Hit, roughness: f32, rando: &mut ThreadRng) -> Ray {
//     let epsilon: f32 = rando.next_u32(); // this doesn't appear to be a float
//     let phi = rando.next_u32() * 2.0 * PI;
//     let inner_sqrt = (epsilon / (1.0 - epsilon)).sqrt();
//     let theta = (roughness * inner_sqrt).arctan();
//     let scaling = 4.0 * half_vector() * output_direction();
//     let dir = make_dir_from_spherical_coords();
//     return Ray::new(hit.point.clone(), dir);
//     // ocnvert these to cartesian ray dir
// }

// fn reflect_ray(ray: &mut Ray, n: &Vector3, specularity: f32, roughness: f32, rand: &ThreadRng) {
//         // // intersection with glossy object, using phong cosine model
//         // let cos_i = (n * -1.0).dot(&ray.dir);
//         // let spec_dir = (ray.dir + n*cos_i * 2.0).normalise();
    
//         // let u_mod = rand.gen::<f32>().powf(1.0 / (specularity + 1.0));
//         // let r = (1.0 - u_mod*u_mod).sqrt();
//         // let phi: f32 = 2.0 * 3.1415926 * rand.gen::<f32>();
//         // let sample_vec = Vector3::new(phi.tan()*r, phi.sin()*r, u_mod);
    
//         // let mut oriented_x: Vector3;
//         // let mut oriented_y: Vector3;
//         // spec_dir.form_ons(&mut oriented_x, &mut oriented_y);
//         // let rotated_dir = Vector3::new(
//         //     Vector3::new(oriented_x.x, oriented_y.x, spec_dir.x).dot(&sample_vec),
//         //     Vector3::new(oriented_x.y, oriented_y.y, spec_dir.y).dot(&sample_vec),
//         //     Vector3::new(oriented_x.z, oriented_y.z, spec_dir.z).dot(&sample_vec)
//         // );
//         // ray.dir = rotated_dir.normalise();
// }

// fn transmit_ray(ray: &mut Ray, n: &Vector3, specularity: f32, r_index: f32, rand: &ThreadRng) {

// }

fn get_explicit_light_contrib(hit: &Hit, scene: &Scene) -> Pixel {
    let mut explicit_light = Pixel::new_black();
    for light in scene.get_lights().iter() {
        let to_light = Ray::new(hit.point, light.get_position() - &hit.point);
        let light_dist = hit.point.distance_to(light.get_position());
        let light_hit = scene.get_closest_intersect(&to_light);
        if light_hit.is_some() && light_dist <= light_hit.unwrap().t {
            let w = (light.get_position() - &hit.point).normalise();
            // TODO the dot product should be replaced by the material's BRDF once you have that going.
            explicit_light = explicit_light + (light.color * hit.normal.dot(&w));
        }
    }
    return explicit_light;
}