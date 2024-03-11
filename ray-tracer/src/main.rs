use std::fs::File;
use std::io::{Write, Result};

fn write_ppm(height: i32, width: i32) -> Result<()> {
    let mut file = File::create("out.ppm")?;

    writeln!(file, "P3\n{} {}\n255\n", width, height)?;
    for i in 0..height {
        let scan_lines = height - i;
        for j in 0..width {
            let r: i32 = ((j as f32 / (width - 1) as f32) * 255.99) as i32;
            let g: i32 = ((i as f32 / (height - 1) as f32) * 255.99) as i32;
            let b: i32 = 0;

            writeln!(file, "{} {} {}", r, g, b)?;
        }
    }
    Ok(())
}

fn main() {
    let height: i32 = 256;
    let width: i32 = 256;

    if let Err(e) = write_ppm(height, width) {
        eprintln!("Error writing to file: {}", e);
    }
}

