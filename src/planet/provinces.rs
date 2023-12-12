use image::{Rgb, RgbImage, Rgba, RgbaImage};
use rand::prelude::*;

pub fn create_province_colors(
    cell_count: usize,
    width: u32,
    height: u32,
) -> Vec<(Rgb<u8>, u32, u32)> {
    let mut used_colors: Vec<(Rgb<u8>, u32, u32)> = Vec::new();
    for _ in 0..cell_count {
        loop {
            let new_x = rand::thread_rng().gen_range(1..=(width - 1));
            let new_y = rand::thread_rng().gen_range(1..=(height - 1));
            let contains_coords = used_colors
                .iter()
                .any(|&(_, ref x, ref y)| *x == new_x && *y == new_y);
            if !contains_coords {
                loop {
                    let r: u8 = rand::thread_rng().gen_range(1..=255);
                    let g: u8 = rand::thread_rng().gen_range(1..=255);
                    let b: u8 = rand::thread_rng().gen_range(1..=255);
                    let new_color = image::Rgb([r, g, b]);
                    let contains_new_color = used_colors
                        .iter()
                        .any(|&(ref color, _, _)| *color == new_color);
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
    return used_colors;
}

pub fn create_provinces_image(
    colors: Vec<(Rgb<u8>, u32, u32)>,
    width: u32,
    height: u32,
) -> RgbImage {
    let mut image: RgbImage = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let mut closest = None;
            let mut min_distance = None;
            for point in &colors {
                let color = point.0;
                let px = point.1;
                let py = point.2;
                let distance =
                    ((px as i32 - x as i32).pow(2) + (py as i32 - y as i32).pow(2)) as f64;
                match min_distance {
                    None => {
                        min_distance = Some(distance);
                        closest = Some(color);
                    }
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

    let mut border_image: RgbaImage = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let current_color = *image.get_pixel(x, y);

            for i in -1..=1 {
                for j in -1..=1 {
                    let nx = x as i32 + i;
                    let ny = y as i32 + j;

                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        continue;
                    }

                    let neighbor_color = *image.get_pixel(nx as u32, ny as u32);

                    if current_color != neighbor_color {
                        *border_image.get_pixel_mut(x, y) = Rgba([0, 0, 0, 255]);
                        break;
                    }
                }
            }
        }
    }
    image.save("assets/saves/output.png").unwrap();
    border_image.save("assets/saves/borders.png").unwrap();
    return image;
}
