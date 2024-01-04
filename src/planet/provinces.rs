use image::{
    imageops, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage,
};
use rand::prelude::*;
use std::f32::consts::PI;

use super::noise;

const DISPLACEMENT_FACTOR: f64 = 86.0;

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
    let noise_map: Vec<Vec<f64>> = noise::make_perlin_noise(width, height);
    for x in 0..width {
        for y in 0..height {
            let mut closest = None;
            let mut min_distance = None;
            for point in &colors {
                let color = point.0;
                let px = point.1;
                let py = point.2;

                let noise_value_x = noise_map[x as usize][y as usize];
                let noise_value_y =
                    noise_map[((x + width / 2) % width as u32) as usize][y as usize];

                let distorted_x = x as f64 + noise_value_x * DISPLACEMENT_FACTOR;
                let distorted_y = y as f64 + noise_value_y * DISPLACEMENT_FACTOR;

                let distance =
                    ((px as f64 - distorted_x).powi(2) + (py as f64 - distorted_y).powi(2)).sqrt();

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
    return image;
}

pub fn get_border_images(width: u32, height: u32, image: &RgbImage) -> Vec<RgbaImage> {
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
    let dynamic_image: DynamicImage = image::DynamicImage::ImageRgba8(border_image);
    let mut border_images: Vec<RgbaImage> = Vec::with_capacity(6);
    for face_id in 0..6 {
        let cube_face = create_cube_map_face(&dynamic_image, face_id, 500, 500);
        border_images.push(cube_face.clone());
        cube_face
            .save(&format!("assets/saves/cube_face_{}.png", face_id))
            .expect("Failed saving image");
    }

    border_images[0] = imageops::rotate270(&border_images[0]);
    border_images[0] = imageops::flip_horizontal(&border_images[0]);
    border_images[1] = imageops::flip_horizontal(&border_images[1]);
    border_images[2] = imageops::rotate90(&border_images[2]);
    border_images[2] = imageops::flip_horizontal(&border_images[2]);
    border_images[3] = imageops::flip_horizontal(&border_images[3]);
    border_images[4] = imageops::flip_horizontal(&border_images[4]);
    border_images[5] = imageops::flip_horizontal(&border_images[5]);
    border_images[5] = imageops::rotate90(&border_images[5]);
    return border_images;
}

fn create_cube_map_face(img: &DynamicImage, face_id: usize, width: u32, height: u32) -> RgbaImage {
    //algorithm found here: https://stackoverflow.com/questions/29678510/convert-21-equirectangular-panorama-to-cube-map
    let face_transforms = [
        (0.0_f32, 0.0_f32),
        (PI / 2.0, 0.0_f32),
        (PI, 0.0_f32),
        (-PI / 2.0, 0.0_f32),
        (0.0_f32, -PI / 2.0),
        (0.0_f32, PI / 2.0),
    ];

    let (ftu, ftv) = face_transforms[face_id];
    let in_width = img.width() as f32;
    let in_height = img.height() as f32;
    let an = (PI / 4.0).sin();
    let ak = (PI / 4.0).cos();

    let mut face = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let ny = y as f32 / height as f32 - 0.5;
            let nx = x as f32 / width as f32 - 0.5;

            let nx = nx * 2.0 * an;
            let ny = ny * 2.0 * an;

            let (u, v) = if ftv == 0.0 {
                let u = nx.atan2(ak);
                let v = (ny * u.cos()).atan2(ak);
                (u + ftu, v)
            } else if ftv > 0.0 {
                let d = (nx * nx + ny * ny).sqrt();
                let v = -((PI / 2.0 - d.atan2(ak)).cos().atan2(d));
                let u = ny.atan2(nx);
                (u, v)
            } else {
                let d = (nx * nx + ny * ny).sqrt();
                let v = (PI / 2.0 - d.atan2(ak)).cos().atan2(d);
                let u = nx.atan2(-ny);
                (u, v)
            };

            let u = u / PI;
            let v = v / (PI / 2.0);

            let u = if u < -1.0 {
                u + 2.0
            } else if u > 1.0 {
                u - 2.0
            } else {
                u
            };
            let v = if v < -1.0 {
                v + 2.0
            } else if v > 1.0 {
                v - 2.0
            } else {
                v
            };

            let u = ((u + 1.0) * 0.5 * (in_width - 1.0)).round() as u32;
            let v = ((v + 1.0) * 0.5 * (in_height - 1.0)).round() as u32;
            let pixel = img.get_pixel(u.min(img.width() - 1), v.min(img.height() - 1));
            face.put_pixel(x, y, pixel.to_rgba());
        }
    }

    return face;
}
