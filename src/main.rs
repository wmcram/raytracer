fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f32) / (image_width - 1) as f32;
            let g = (j as f32) / (image_height - 1) as f32;
            let b: f32 = 0.0;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            print!("{ir} {ig} {ib}\n");
        }
    }
}
