use bevy::math::Vec2;
use std::f32::consts::PI;

const GRID_SIZE: i32 = 400;

pub fn make_perlin_noise(width: u32, height: u32) -> Vec<Vec<f64>> {
    let mut noise_map = vec![vec![0.0; height as usize]; width as usize];

    for x in 0..width {
        for y in 0..height {
            let mut val: f32 = 0.0;
            let mut freq: f32 = 1.0;
            let mut amp: f32 = 1.0;

            for _ in 0..12 {
                val += perlin(
                    (x as f32 * freq) / GRID_SIZE as f32,
                    (y as f32 * freq) / GRID_SIZE as f32,
                ) * amp;
                freq *= 2.0;
                amp /= 2.0;
            }

            val *= 1.2;
            if val >= 1.0 {
                val = 1.0;
            } else if val <= -1.0 {
                val = -1.0;
            }

            noise_map[x as usize][y as usize] = val as f64;
        }
    }

    return noise_map;
}

fn random_gradient(ix: i32, iy: i32) -> Vec2 {
    const W: u32 = 8 * std::mem::size_of::<u32>() as u32;
    const S: u32 = W / 2;
    let mut a = ix as u32;
    let mut b = iy as u32;

    a = a.wrapping_mul(3284157443);

    //Black magic from C, go see this for reference:
    //https://pastebin.com/XwCPn0xR
    b ^= a << S | a >> W - S;
    b = b.wrapping_mul(1911520717);

    a ^= b << S | b >> W - S;
    a = a.wrapping_mul(2048419325);

    let random = a as f32 * (PI / !(0u32.wrapping_sub(1) >> 1) as f32);

    let v = Vec2 {
        x: random.sin(),
        y: random.cos(),
    };

    return v;
}

fn dot_grid_gradient(ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    let gradient: Vec2 = random_gradient(ix, iy);
    let dx: f32 = x - ix as f32;
    let dy: f32 = y - iy as f32;
    return dx * gradient.x + dy * gradient.y;
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}

fn perlin(x: f32, y: f32) -> f32 {
    let x0: i32 = x as i32;
    let y0: i32 = y as i32;
    let x1: i32 = x0 + 1;
    let y1: i32 = y0 + 1;

    let sx: f32 = x - x0 as f32;
    let sy: f32 = y - y0 as f32;

    let mut n0: f32 = dot_grid_gradient(x0, y0, x, y);
    let mut n1: f32 = dot_grid_gradient(x1, y0, x, y);
    let ix0: f32 = interpolate(n0, n1, sx);

    n0 = dot_grid_gradient(x0, y1, x, y);
    n1 = dot_grid_gradient(x1, y1, x, y);
    let ix1: f32 = interpolate(n0, n1, sx);

    let value: f32 = interpolate(ix0, ix1, sy);
    return value;
}
