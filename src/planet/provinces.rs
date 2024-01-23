use image::{
    imageops::{self, blur},
    Rgb, RgbImage, Rgba, RgbaImage,
};
use rand::prelude::*;

use super::noise;

const DISPLACEMENT_FACTOR: f64 = 84.0;

pub fn create_province_colors(cell_count: usize, dimensions: u32) -> Vec<(Rgb<u8>, u32, u32, u32)> {
    let mut used_colors: Vec<(Rgb<u8>, u32, u32, u32)> = Vec::new();
    for _ in 0..cell_count {
        loop {
            let new_x = rand::thread_rng().gen_range(1..=(dimensions - 1));
            let new_y = rand::thread_rng().gen_range(1..=(dimensions - 1));
            let new_z = rand::thread_rng().gen_range(1..=(dimensions - 1));
            let contains_coords = used_colors
                .iter()
                .any(|&(_, ref x, ref y, ref z)| *x == new_x && *y == new_y && *z == new_z);
            if !contains_coords {
                loop {
                    let r: u8 = rand::thread_rng().gen_range(1..=255);
                    let g: u8 = rand::thread_rng().gen_range(1..=255);
                    let b: u8 = rand::thread_rng().gen_range(1..=255);
                    let new_color = image::Rgb([r, g, b]);
                    let contains_new_color = used_colors
                        .iter()
                        .any(|&(ref color, _, _, _)| *color == new_color);
                    if !contains_new_color {
                        used_colors.push((new_color, new_x, new_y, new_z));
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

pub fn create_provinces_images(
    colors: Vec<(Rgb<u8>, u32, u32, u32)>,
    dimensions: u32,
) -> Vec<RgbImage> {
    let noise_map: Vec<Vec<Vec<f64>>> = noise::make_perlin_noise(dimensions);
    let mut voronoi_faces: Vec<RgbImage> = Vec::with_capacity(6);
    for face_index in 0..6 {
        let mut image: RgbImage = RgbImage::new(dimensions, dimensions);
        let mut previous_noise: f64 = 0.0;
        for x in 0..dimensions {
            for y in 0..dimensions {
                let (nx, ny, nz) = match face_index {
                    0 => ((dimensions - 1) as f64, x as f64, y as f64), // Positive X
                    1 => (0.0, x as f64, y as f64),                     // Negative X
                    2 => (x as f64, (dimensions - 1) as f64, y as f64), // Positive Y
                    3 => (x as f64, 0.0, y as f64),                     // Negative Y
                    4 => (x as f64, y as f64, (dimensions - 1) as f64), // Positive Z
                    _ => (x as f64, y as f64, 0.0),                     // Negative Z
                };
                let mut noise_value = noise_map[nx as usize][ny as usize][nz as usize];
                if noise_value.is_nan() {
                    noise_value = previous_noise;
                }
                previous_noise = noise_value;
                let distorted_x = nx + noise_value * DISPLACEMENT_FACTOR;
                let distorted_y = ny + noise_value * DISPLACEMENT_FACTOR;
                let distorted_z = nz + noise_value * DISPLACEMENT_FACTOR;
                let mut closest = None;
                let mut min_distance = f64::MAX;
                for (color, px, py, pz) in &colors {
                    let distance = ((distorted_x - *px as f64).powi(2)
                        + (distorted_y - *py as f64).powi(2)
                        + (distorted_z - *pz as f64).powi(2))
                    .sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                        closest = Some(*color);
                    }
                }
                if let Some(color) = closest {
                    *image.get_pixel_mut(x, y) = color;
                }
            }
        }
        voronoi_faces.push(image);
    }
    voronoi_faces[0] = imageops::flip_horizontal(&voronoi_faces[0]);
    voronoi_faces[2] = imageops::rotate270(&voronoi_faces[2]);
    voronoi_faces[3] = imageops::flip_horizontal(&voronoi_faces[3]);
    voronoi_faces[3] = imageops::rotate90(&voronoi_faces[3]);
    voronoi_faces[4] = imageops::flip_horizontal(&voronoi_faces[4]);
    return voronoi_faces;
}

pub fn get_colors(images: &Vec<RgbImage>) -> Vec<Rgb<u8>> {
    let mut colors: Vec<Rgb<u8>> = Vec::new();
    for image in images {
        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = image.get_pixel(x, y);
                if !colors.contains(pixel) {
                    colors.push(pixel.clone());
                }
            }
        }
    }
    return colors;
}

pub fn get_border_images(dimensions: u32, images: &Vec<RgbImage>) -> Vec<RgbaImage> {
    let mut border_images: Vec<RgbaImage> = Vec::with_capacity(images.len());
    let alpha = 180;
    let sigma = 0.675;
    let bloom_factor = 1.5;
    for image in images {
        let mut border_image: RgbaImage = RgbaImage::new(dimensions, dimensions);
        for x in 0..dimensions {
            for y in 0..dimensions {
                let current_color = *image.get_pixel(x, y);
                for i in -1..=1 {
                    for j in -1..=1 {
                        let nx: u32 = (x as i32 + i) as u32;
                        let ny: u32 = (y as i32 + j) as u32;
                        if nx >= dimensions || ny >= dimensions {
                            continue;
                        }
                        let neighbor_color = *image.get_pixel(nx, ny);
                        if current_color != neighbor_color {
                            // Applies a bloom effect to the border pixel
                            // Maybe could be sped up using a frag shader but since it only affects load time
                            // there shouldn't be a large performance hit
                            let original_color = *image.get_pixel(x, y);
                            let new_rgba = [
                                (original_color[0] as f32 * bloom_factor).min(255.0) as u8,
                                (original_color[1] as f32 * bloom_factor).min(255.0) as u8,
                                (original_color[2] as f32 * bloom_factor).min(255.0) as u8,
                                alpha,
                            ];
                            *border_image.get_pixel_mut(x, y) = Rgba(new_rgba);
                        }
                    }
                }
            }
        }
        border_images.push(border_image);
    }

    // perform a Gaussian blur on each border image to fix jagged lines
    for border_image in &mut border_images {
        *border_image = blur(border_image, sigma);
    }

    return border_images;
}



