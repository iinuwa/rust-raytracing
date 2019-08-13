mod camera;
mod objects;
mod ray;
mod vec3;

use camera::Camera;
use objects::{HitRecord, Hittable, HittableList, Sphere};
use rand::prelude::*;
use ray::Ray;
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

    let camera = Camera::new();
    let sphere1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let world = HittableList::new(vec![&sphere1, &sphere2]);

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
                color += calculate_color(&my_ray, &world);
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

fn calculate_color(ray: &Ray, world: &Hittable) -> Vec3 {
    if let Some(hit_record) = world.hit(&ray, 0.001, f32::MAX) {
        return shade(world, hit_record);
    }
    linear_blend(ray)
}

fn linear_blend(ray: &Ray) -> Vec3 {
    let unit_direction = &ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn shade(world: &Hittable, hit_record: HitRecord) -> Vec3 {
    let target = &hit_record.point + &hit_record.normal + random_in_unit_sphere();

    0.5 * calculate_color(&Ray::new(hit_record.point.clone(), target - hit_record.point), world )
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point: Vec3;
    while { 
        point = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3(1.0, 1.0, 1.0);
        point.squared_length() >= 1.0
    }{}
    point
}