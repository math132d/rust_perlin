mod perlin;

use std::env;
use std::path;
use perlin::Perlin2D;

// rust_perlin image.png 256 256

fn main() {
    let mut args = env::args();

    if args.len() < 4 { return }

    let path = args.nth(1).unwrap();
    let width = args.nth(2).unwrap().parse::<u32>().unwrap();
    let height = args.nth(3).unwrap().parse::<u32>().unwrap();

    if path::Path::new(&path[..]).exists() {
        println!("File exsists! Aborting");
        return;
    }

    //perlin_image(width, height, 64, 4).save(path).unwrap();
}

fn perlin_image(width: u32, height: u32, frequency: u32, octaves: u8) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let perlin = Perlin2D::new(
        width/frequency,
        height/frequency,
        octaves,
    );

    let img_buf = image::ImageBuffer::from_fn(width, height, |x, y| {
        
        #[allow(unused_parens)]
        let gray = perlin.noise(
            (x as f32 / width as f32 * 4.0),
            (y as f32 / height as f32 * 4.0),
        );

        let gray = (gray * 255.0).round() as u8;
    
        image::Luma([gray])
    });

    img_buf
}