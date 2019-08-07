mod vec3;
mod ray;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use vec3::{Color, Vec3, Vector};
use ray::Ray;

fn main() -> io::Result<()> {
    let x_px = 200;
    let y_px = 100;
    let f = File::create("foo.ppm")?;
    let mut output = String::new();
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    //Header
    header(&mut output, &x_px, &y_px);
    // Body
    for j in (0..y_px).rev() {
        for i in 0..x_px {
            let u = i as f32 / x_px as f32;
            let v = j as f32 / y_px as f32;
            let direction = &lower_left_corner + &horizontal*u + &vertical*v;
            let my_ray = Ray {
                point: &origin,
                vector: direction,
            };
            let color = linear_blend(&my_ray);
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

fn header(output: &mut String, width: &usize, height: &usize) {
    output.push_str("P3\n");
    output.push_str(&width.to_string());
    output.push_str(" ");
    output.push_str(&height.to_string());
    output.push_str("\n255\n");

}

fn linear_blend(ray: &Ray<f32>) -> Vec3<f32> {

    let unit_direction = &ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t)  + Vec3(0.5, 0.7, 1.0) * t
}