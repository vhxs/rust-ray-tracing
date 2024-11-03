pub fn run() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    print!(
        "P3\n{image_width} {image_height}\n255\n",
        image_width = image_width,
        image_height = image_height
    );

    for j in 0..image_height {
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width - 1) as f32;
            let g: f32 = j as f32 / (image_height - 1) as f32;
            let b: f32 = 0.0;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            print!("{ir} {ig} {ib}\n", ir = ir, ig = ig, ib = ib);
        }
    }
}
