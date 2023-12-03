use image::{Pixel, Rgb, RgbImage};
use rand::prelude::*;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;
const CELL_COUNT: usize = 50;

pub fn create_provinces_image() {
    let mut used_colors: Vec<(Rgb<u8>, u32, u32)> = Vec::new();
    let mut image: RgbImage = RgbImage::new(WIDTH, HEIGHT);
    for _ in 0..CELL_COUNT {
        loop {
            let new_x = rand::thread_rng().gen_range(1..=(WIDTH-1));
            let new_y = rand::thread_rng().gen_range(1..=(HEIGHT-1));
            let contains_coords = used_colors.iter().any(|&(_, ref x, ref y)| *x == new_x && *y == new_y);
            if !contains_coords {
                loop {
                    let r: u8 = rand::thread_rng().gen_range(1..=255);
                    let g: u8 = rand::thread_rng().gen_range(1..=255);
                    let b: u8 = rand::thread_rng().gen_range(1..=255);
                    let new_color = image::Rgb([r, g, b]);
                    let contains_new_color = used_colors.iter().any(|&(ref color, _, _)| *color == new_color);
                    if !contains_new_color {
                        used_colors.push((new_color, new_x, new_y));
                        break;
                    }
                }
            } else {
                continue;
            }
            break;
        }
    }
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut closest = None;
            let mut min_distance = None;
            for point in &used_colors {
                let color = point.0;
                let px = point.1;
                let py = point.2;
                let distance = ((px as i32 - x as i32).pow(2) + (py as i32 - y as i32).pow(2)) as f64;
                match min_distance {
                    None => {
                        min_distance = Some(distance);
                        closest = Some(color);
                    },
                    Some(d) => {
                        if distance < d {
                            min_distance = Some(distance);
                            closest = Some(color);
                        }
                    }
                }
            }
            if let Some(color) = closest {
                *image.get_pixel_mut(x, y) = color;
            }
        }
    }
    image.save("output.png").unwrap();
}