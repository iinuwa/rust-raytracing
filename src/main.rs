use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

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
            let r = i as f32 / x_px as f32;
            let g = j as f32 / y_px as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u32;
            let ig = (255.9 * g) as u32;
            let ib = (255.9 * b) as u32;
            output.push_str(&ir.to_string());
            output.push_str(" ");
            output.push_str(&ig.to_string());
            output.push_str(" ");
            output.push_str(&ib.to_string());
            output.push_str("\n");
        }
    }

    {
        let mut writer = BufWriter::new(f);

        writer.write(&output.as_bytes())?;
    }

    Ok(())
}
