mod color;
mod vec3;

use std::io::stdout;

use color::Color;

use crate::color::write_color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.0,
            );
            write_color(&mut stdout(), &pixel_color);
        }
    }
    eprint!("\rDone.                \n")
}
