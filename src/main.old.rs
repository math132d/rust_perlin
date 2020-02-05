extern crate image;
extern crate rand;

mod vector_2d;

use rand::prelude::*;
use vector_2d::Vector2D;

const IMG_W : u32 = 256;
const IMG_H : u32 = 256;

const SUBDIV_W : u32 = 16;
const SUBDIV_H : u32 = 16;

struct GradientMap {
    width: u32,
    height: u32,
    map: Vec<Vector2D>
}

impl GradientMap {

    fn new(width: u32, height: u32) -> GradientMap {
        GradientMap {
            width, 
            height,
            map: Vec::with_capacity((width * height) as usize),
        }
    }

    fn get_vector(&self, x: u32, y: u32) -> Option<&Vector2D> {
        let index: usize = (x + y*self.width) as usize;
        self.map.get(index)
    }
}


fn main() {

    let BLOCK_W = IMG_W/SUBDIV_W;
    let BLOCK_H = IMG_H/SUBDIV_H;

    let mut gradients = GradientMap::new(SUBDIV_W+1, SUBDIV_H+1);

    for _ in 0..gradients.map.capacity() {
        let rdm_x : f32 = random::<f32>() * 2f32 -1f32;
        let rdm_y : f32 = random::<f32>() * 2f32 -1f32;

        gradients.map.push(Vector2D {
            x: rdm_x,
            y: rdm_y,
        });
    }
2
    let img_buf = image::ImageBuffer::from_fn(IMG_W, IMG_H, |x, y| {

        let pixel_to_grid_x = x as f32 / BLOCK_W as f32;
        let pixel_to_grid_y = y as f32 / BLOCK_H as f32;

        let closest_grid = gradients.get_vector(
            pixel_to_grid_x.round() as u32,
            pixel_to_grid_y.round() as u32,
        ).unwrap();

        let grid_coord = Vector2D {
            x: pixel_to_grid_x.round(),
            y: pixel_to_grid_y.round(),
        };

        let grid_to_point = grid_coord.vec_to( &Vector2D {x: pixel_to_grid_x, y: pixel_to_grid_y } );

        let gray_value : u8 = (((grid_to_point.dot(&closest_grid) + 1.0) / 2.0) * 255.0) as u8;


        image::Luma([gray_value])
    });

    match img_buf.save("image.png") {
        Ok(_) => println!("Saved image successfully"),
        Err(e) => println!("{}", e),
    }
}