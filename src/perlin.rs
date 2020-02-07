extern crate rand;

use std::thread::{self, JoinHandle};
use std::sync::Arc;
use rand::Rng;
use crate::vector_2d::Vector2D;

pub struct Perlin2D {
    freq: u32,
    size: u32,
    octaves: u32,
    map: Vec<Vector2D>
}

const GAIN: f32 = 0.4;
const RAND_VECTORS: [Vector2D; 24] = [  //Gen one random number per vector instead of two.
    Vector2D { x: 1.000, y: 0.000},     //Lookup in this list.
    Vector2D { x: 0.966, y: 0.259},
    Vector2D { x: 0.866, y: 0.500},
    Vector2D { x: 0.707, y: 0.707},
    Vector2D { x: 0.500, y: 0.866},
    Vector2D { x: 0.259, y: 0.966},
    Vector2D { x: 0.000, y: 1.000},
    Vector2D { x: -0.259, y: 0.966},
    Vector2D { x: -0.500, y: 0.866},
    Vector2D { x: -0.707, y: 0.707},
    Vector2D { x: -0.866, y: 0.500},
    Vector2D { x: -0.966, y: 0.259},
    Vector2D { x: -1.000, y: 0.000},
    Vector2D { x: -0.966, y: -0.259},
    Vector2D { x: -0.866, y: -0.500},
    Vector2D { x: -0.707, y: -0.707},
    Vector2D { x: -0.500, y: -0.866},
    Vector2D { x: -0.259, y: -0.966},
    Vector2D { x: 0.000, y: -1.000},
    Vector2D { x: 0.259, y: -0.966},
    Vector2D { x: 0.500, y: -0.866},
    Vector2D { x: 0.707, y: -0.707},
    Vector2D { x: 0.866, y: -0.500},
    Vector2D { x: 0.966, y: -0.259},
];

impl Perlin2D {

    pub fn new(freq: u32, octaves: u32) -> Perlin2D {
        let size = freq * 2u32.pow(octaves as u32);

        let mut map = Vec::with_capacity((size * size) as usize);

        let mut rng = rand::thread_rng();

        for _ in 0..map.capacity() {
            map.push(RAND_VECTORS[rng.gen_range(0, 24)].clone());
        }

        Perlin2D {
            freq,
            size,
            octaves,
            map,
        }
    }

    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let mut gray: f32 = 0.0;

        let x = (x - x.floor()) as f32;
        let y = (y - y.floor()) as f32;

        for i in 0..self.octaves {
            let octave_size = self.freq * 2u32.pow(i);

            let mut octave = self.basic_noise(
                x as f32,
                y as f32,
                octave_size,
                octave_size,
            );

            octave *= GAIN.powi((i+1) as i32);

            gray += octave;
        }

        return gray;
    }


    pub fn basic_noise(&self, x: f32, y: f32, gx: u32, gy: u32) -> f32 {
        //Returns noise sampled from p(x, y) between 0 and 1

        //Wrapping x and y to btwn 0 and 1 -> Scales to size
        let x = (x - x.floor()) * gx as f32;
        let y = (y - y.floor()) * gy as f32;

        let x0: u32 = x.floor() as u32;
        let y0: u32 = y.floor() as u32;
        let x1: u32 = x0 + 1;
        let y1: u32 = y0 + 1;

        let sx: f32 = x - x0 as f32;
        let sy: f32 = y - y0 as f32;

        // Modified coordinates for left/bottom edge case.
        // Enables tiling of noise
        let x1_m = if x1 > (gx-1) { 0 } else { x1 };
        let y1_m = if y1 > (gy-1) { 0 } else { y1 };

        //Top Corners
        let a_vec = self.get_vector(x0, y0).unwrap();
        let b_vec = self.get_vector(x1_m, y0).unwrap();

        let a_dot = a_vec.dot_fast( &Vector2D {x: x - x0 as f32, y: y - y0 as f32} );
        let b_dot = b_vec.dot_fast( &Vector2D {x: x - x1 as f32, y: y - y0 as f32} );
        let top_dot = smooth_interpolation(a_dot, b_dot, sx);

        //Bottom Corners
        let a_vec = self.get_vector(x0, y1_m).unwrap();
        let b_vec = self.get_vector(x1_m, y1_m).unwrap();

        let a_dot = a_vec.dot_fast( &Vector2D {x: x - x0 as f32, y: y - y1 as f32} );
        let b_dot = b_vec.dot_fast( &Vector2D {x: x - x1 as f32, y: y - y1 as f32} );
        let btm_dot = smooth_interpolation(a_dot, b_dot, sx);


        return  (smooth_interpolation(top_dot, btm_dot, sy) + 1.0) / 2.0;
    }

    fn get_vector(&self, x: u32, y: u32) -> Option<&Vector2D> {
        let index: usize = (x + y*self.size) as usize;
        self.map.get(index)
    }
}

pub fn perlin_image(width: u32, height: u32, frequency: u32, octaves: u32) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    
    let shortest_side = if width <= height { width } else { height };
    
    let perlin : Arc<Perlin2D> = Arc::new(
        Perlin2D::new(
            frequency,
            octaves,
        )
    );

    let mut raw_pixels : Vec<u8> = Vec::with_capacity((width * height) as usize);

    let thread_count = 8;
    let pixels_per_thread = raw_pixels.capacity() / thread_count;
    let mut thread_handles : Vec<JoinHandle<Vec<u8>>> = Vec::with_capacity(4);

    for thread_idx in 0..thread_handles.capacity() {

        let thread_perlin = Arc::clone(&perlin);

        let thread = thread::spawn(move || {
            let mut raw_pixel_section : Vec<u8> = Vec::with_capacity(pixels_per_thread);

            let pixel_range = std::ops::Range {
                start:  pixels_per_thread * thread_idx,
                end:    pixels_per_thread * (thread_idx+1)
            };

            for pixel_idx in pixel_range {

                let x = pixel_idx % width as usize;
                let y = pixel_idx / width as usize;

                let gray = thread_perlin.noise(
                    x as f32 / shortest_side as f32,
                    y as f32 / shortest_side as f32,
                );
        
                raw_pixel_section.push(
                    (gray * 255.0).round() as u8
                );
            }

            raw_pixel_section
        });

        thread_handles.push(thread);
    }

    thread_handles.reverse();

    while thread_handles.len() > 0 {
        let handle = thread_handles.pop().unwrap();

        let mut raw_pixel_section = handle.join().unwrap();

        raw_pixels.append(&mut raw_pixel_section);
    }

    let img_buf: Option<image::ImageBuffer<image::Luma<u8>, _>> = image::ImageBuffer::from_raw(width, height, raw_pixels);

    match img_buf {
        Some(image) => image,
        None => panic!("Couldn't create image from container!"),
    }
}

fn smooth_interpolation(a: f32, b: f32, s: f32) -> f32{
    a + smoothstep(s) * (b-a)
}

fn smoothstep(x: f32) -> f32{
    x * x * (3.0 - 2.0 * x)
}