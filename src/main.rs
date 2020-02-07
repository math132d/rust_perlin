extern crate clap;

mod vector_2d;
mod perlin;

use std::process;
use std::time::Instant;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Perlin Generator")
                        .version("1.0")
                        .arg(Arg::with_name("Width")
                            .index(1)
                            .required(true))
                        .arg(Arg::with_name("Height")
                            .index(2)
                            .required(true))
                        .arg(Arg::with_name("Path")
                            .index(3)
                            .required(true))
                        .arg(Arg::with_name("Frequency")
                            .short("f")
                            .takes_value(true))
                        .arg(Arg::with_name("Octaves")
                            .short("o")
                            .takes_value(true))
                    .get_matches();

    let width: u32 = match matches.value_of("Width").unwrap().parse::<u32>() {
        Ok(width) => width,
        Err(err) => panic!(err),
    };

    let height: u32 = match matches.value_of("Height").unwrap().parse::<u32>() {
        Ok(height) => height, 
        Err(err) => panic!(err),
    };

    let freq: u32 = match matches.value_of("Frequency") {
        Some(x) => { 
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    println!("Frequency must be a valid number!");
                    4
                },
            }
        },
        None => 4,
    };

    let oct: u32 = match matches.value_of("Octaves") {
        Some(x) => { 
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => {
                    println!("Octaves must be a valid number!");
                    1
                },
            }
        },
        None => 1,
    };

    let path: &str = match matches.value_of("Path") {
        Some(path) => {
            if Path::new(path).exists() {
                println!("Image '{}' already exists, choose another name!", path);
                process::exit(1);
            }else{
                path
            }
        },
        None => panic!("Requre path to run")
    };

    let start = Instant::now();

    perlin::perlin_image(width, height, freq, oct).save(path).unwrap();

    println!("Generated image in {} ms - Saved as '{}'", Instant::now().duration_since(start).as_millis(), path);
}