mod vec3;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use vec3::{Color, Vec3};

fn main() -> io::Result<()> {
    let x_px = 200;
    let y_px = 100;
    let f = File::create("foo.ppm")?;
    let mut output = String::new();

    //Header
    output.push_str("P3\n");
    output.push_str(&x_px.to_string());
    output.push_str(" ");
    output.push_str(&y_px.to_string());
    output.push_str("\n255\n");

    // Body
    for j in (0..y_px).rev() {
        for i in 0..x_px {
            let color = Vec3(i / x_px, j / y_px, 0.2);
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
