/*
extern crate image;
extern crate rayon;

use std::io::{self, Write};
use std::ops::Range;

use rayon::prelude::*;

use image::GenericImage;
use image::Pixels;
use image::Pixel;

*/
extern crate rand;
use rand::random;

const N: usize = 32;
const NEVER: usize = 9_999_999;

fn main() {

    let mut v = Vec::with_capacity(N);
    for _ in 0..N {
        v.push(random::<bool>());
    }
    let mut i = 0;
    while notall(&v) {
        let take_from = random::<usize>() % N;
        v[i % N] = v[take_from];
        //v[i % N] = random();
        i += 1;
        if i == NEVER {
            break;
        }
    }
    if i < NEVER {
        println!("Diverged after {} iterations", i);
    } else {
        println!("Never diverged");
    }
}

fn notall(v: &[bool]) -> bool {
    if v.iter().find(|&&x| x == true).is_none() {
        println!("All false");
        false
    } else if v.iter().find(|&&x| x == false).is_none() {
        println!("All true");
        false
    } else {
        true
    }
}

