mod camera;
mod materials;
mod objects;
mod ray;
mod vec3;

use camera::Camera;
use materials::{Dielectric, Lambertian, Material, Metal};
use objects::{Hittable, HittableList, Sphere};
use rand::prelude::*;
use ray::Ray;
use std::cmp::Ordering;
use std::f32;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use vec3::{Color, Vec3, Vector};

fn main() -> io::Result<()> {
    let x_px = 200;
    let y_px = 100;
    let samples = 100;
    let f = File::create("foo.ppm")?;
    let mut output = String::new();

    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let focus_distance = (&look_from - &look_at).length();
    let aperture = 2.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        90.0,
        x_px as f32 / y_px as f32,
        aperture,
        focus_distance,
    );
    // TODO: Why does this need a type annotation but not the rest?
    let sphere1: Sphere<Lambertian> = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: &Lambertian::new(Vec3(0.1, 0.2, 0.5)),
    };
    let sphere2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: &Lambertian::new(Vec3(0.8, 0.8, 0.0)),
    };
    let sphere3 = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Metal::new(Vec3(0.8, 0.6, 0.2), 0.0),
    };
    let sphere4 = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: &Dielectric::new(1.5),
    };
    let sphere5 = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: &Dielectric::new(1.5),
    };
    let world = HittableList::new(vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5]);

    //Header
    header(&mut output, x_px, y_px);
    // Body
    let mut rng = rand::thread_rng();
    for j in (0..y_px).rev() {
        for i in 0..x_px {
            let mut color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u_jitter: f32 = rng.gen();
                let v_jitter: f32 = rng.gen();
                let u = (i as f32 + u_jitter) / x_px as f32;
                let v = (j as f32 + v_jitter) / y_px as f32;
                let my_ray = camera.get_ray(u, v);
                color += calculate_color(&my_ray, &world, 0);
            }
            color /= samples as f32;
            color = Vec3(color.0.sqrt(), color.1.sqrt(), color.2.sqrt());
            let pixel = color * 255.99;

            output.push_str(&pixel.r().to_string());
            output.push_str(" ");
            output.push_str(&pixel.g().to_string());
            output.push_str(" ");
            output.push_str(&pixel.b().to_string());
            output.push_str("\n");
        }
    }

    {
        let mut writer = BufWriter::new(f);

        writer.write_all(&output.as_bytes())?;
    }

    Ok(())
}

fn header(output: &mut String, width: usize, height: usize) {
    output.push_str("P3\n");
    output.push_str(&width.to_string());
    output.push_str(" ");
    output.push_str(&height.to_string());
    output.push_str("\n255\n");
}

fn calculate_color<T>(ray: &Ray, world: &dyn Hittable<T>, depth: usize) -> Vec3 {
    if let Some(hit_record) = world.hit(&ray, 0.001, f32::MAX) {
        let scatter_result = hit_record.material.scatter(ray, hit_record);
        if depth < 50 && scatter_result.is_some() {
            let result = scatter_result.unwrap();
            return result.attenuation
                * calculate_color(&result.scattered_direction, world, depth + 1);
        } else {
            return Vec3(0.0, 0.0, 0.0);
        }
    }
    linear_blend(ray)
}

fn linear_blend(ray: &Ray) -> Vec3 {
    let unit_direction = &ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn random_scene<T>(world: &mut Vec<&Sphere<&dyn Material<T>>>) {
    let number_of_spheres = 500;
    //let world = Vec::with_capacity(number_of_spheres);
    let mut rng = rand::thread_rng();
    for sphere in world.iter() {
        for a in -11..11 {
            for b in -11..11 {
                let material_choice: f32 = rng.gen();
                let x_rand: f32 = rng.gen();
                let z_rand: f32 = rng.gen();
                let center = Vec3(a as f32 + 0.9 * x_rand, 0.2, b as f32 + 0.9 * z_rand);
                if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                    if material_choice < 0.8 {
                        sphere = &Sphere {
                            // diffuse,
                            center,
                            radius: 0.2,
                            material: &Lambertian::new(Vec3(
                                rng.gen() * rng.gen(),
                                rng.gen() * rng.gen(),
                                rng.gen() * rng.gen(),
                            )),
                        };
                    } else if material_choice < 0.95 {
                        // metal
                        sphere = &Sphere {
                            center,
                            radius: 0.2,
                            material: &Metal::new(
                                Vec3(
                                    0.5 * (1.0 + rng.gen()),
                                    0.5 * (1.0 + rng.gen()),
                                    0.5 * (1.0 * rng.gen()),
                                ),
                                0.5 * rng.gen(),
                            ),
                        }
                    } else {
                        // glass
                        sphere = &Sphere {
                            center,
                            radius: 0.2,
                            material: &Dielectric::new(1.5),
                        };
                    }
                }
            }
        }
    }
}
