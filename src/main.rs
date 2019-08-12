mod vec3;
mod ray;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use vec3::{Color, Vec3, Vector};
use ray::Ray;
use std::cmp::Ordering;

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
            let horizontal_vector = u * &horizontal;
            let vertical_vector = v * &vertical;
            let direction = &lower_left_corner + &horizontal_vector + vertical_vector;
            let my_ray = Ray::new(&origin, &direction);
            let color: Vec3 = calculate_color(&my_ray);
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

fn calculate_color(ray: &Ray) -> Vec3 {
    let sphere = Sphere { 
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let shade_factor = hit_sphere(&sphere.center, sphere.radius, ray);
    if shade_factor > 0.0 {
        return shade(shade_factor, ray);
    }
    linear_blend(ray)
}

fn linear_blend(ray: &Ray) -> Vec3 {

    let unit_direction = &ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t)  + Vec3(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let origin_offset = ray.origin() - center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(&origin_offset, ray.direction());
    let c = Vec3::dot(&origin_offset, &origin_offset) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    match discriminant.partial_cmp(&0_f32).unwrap() {
        Ordering::Less => -1.0,
        Ordering::Greater => (-b - discriminant.sqrt()) / (2.0 * a),
        Ordering::Equal => (-b - discriminant.sqrt()) / (2.0 * a),
    }
}

fn shade(factor: f32, ray: &Ray) -> Vec3 { 
    let surface_normal = (ray.point_at(factor) - Vec3(0.0, 0.0, -1.0)).unit_vector();
    0.5 * Vec3(surface_normal.x() + 1.0, surface_normal.y() + 1.0, surface_normal.z() + 1.0)
}
struct Sphere {
    center: Vec3,
    radius: f32, 
}