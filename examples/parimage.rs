extern crate image;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;

use image::ImageBuffer;
use image::Rgb;
use image::Pixel;
use rand::random;

const DIM: u32 = 1024;

fn noise(_x: u32, _y: u32) -> Rgb<u8> {
    let r: u32 = random();
    let data = [ (r >> 24 & 0xff) as u8, (r >> 16 & 0xff) as u8, (r & 0xff) as u8 ];
    Rgb { data }
}

fn avg(vals: &[u8]) -> u8 {
    let count: u32 = vals.len() as u32;
    let sum: u32 = vals.iter().map(|x| *x as u32).sum();
    let avg = sum / count;
    avg as u8
}

fn smooth_pixel(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, pixel: &Rgb<u8>) -> Rgb<u8> {
    let mut neighbors = Vec::with_capacity(9);
    if x > 0 {
        if y > 0 { neighbors.push(img.get_pixel(x - 1, y - 1)); }
        neighbors.push(img.get_pixel(x - 1, y));
        if y < DIM - 1 { neighbors.push(img.get_pixel(x - 1, y + 1)); }
    }
    if y > 0 { neighbors.push(img.get_pixel(x, y - 1)); }
    neighbors.push(img.get_pixel(x, y));
    if y < DIM - 1 { neighbors.push(img.get_pixel(x, y + 1)); }
    if x < DIM - 1 {
        if y > 0 { neighbors.push(img.get_pixel(x + 1, y - 1)); }
        neighbors.push(img.get_pixel(x + 1, y));
        if y < DIM - 1 { neighbors.push(img.get_pixel(x + 1, y + 1)); }
    }
    neighbors.push(pixel);

    let mut reds = Vec::with_capacity(9);
    let mut greens = Vec::with_capacity(9);
    let mut blues = Vec::with_capacity(9);
    for &Rgb { data } in neighbors {
        reds.push(data[0] as u8);
        greens.push(data[1] as u8);
        blues.push(data[2] as u8);
    }
    Rgb { data: [avg(&reds[..]), avg(&greens[..]), avg(&blues[..])] }
}

fn main() {
    let mut img = ImageBuffer::from_fn(DIM, DIM, noise);
    let outimg = ImageBuffer::from_fn(DIM, DIM, |x, y| {
       let pixel = img.get_pixel(x, y);
       smooth_pixel(&img, x, y, pixel)
    });
    img.save("noise.png").expect("start file");
    outimg.save("bounded.png").expect("saving file");
}
