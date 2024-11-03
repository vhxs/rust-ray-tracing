use crate::utils::color::{write_color, Color};

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
            let pixel_color = Color {
                x: (i as f64) / (image_width as f64 - 1.),
                y: (j as f64) / (image_height as f64 - 1.),
                z: 0.,
            };

            write_color(&pixel_color);
        }
    }
}
