use bevy::math::Vec3;

const GRID_SIZE: i32 = 400;

pub fn make_perlin_noise(dimensions: u32) -> Vec<Vec<Vec<f64>>> {
    let mut noise_map =
        vec![vec![vec![0.0; dimensions as usize]; dimensions as usize]; dimensions as usize];

    for x in 0..dimensions {
        for y in 0..dimensions {
            for z in 0..dimensions {
                let mut val: f32 = 0.0;
                let mut freq: f32 = 1.0;
                let mut amp: f32 = 1.0;

                for _ in 0..8 {
                    val += perlin_3d(
                        (x as f32 * freq) / GRID_SIZE as f32,
                        (y as f32 * freq) / GRID_SIZE as f32,
                        (z as f32 * freq) / GRID_SIZE as f32,
                    ) * amp;
                    freq *= 2.0;
                    amp /= 2.0;
                }

                val *= 1.2;
                if val > 1.0 {
                    val = 1.0;
                } else if val < -1.0 {
                    val = -1.0;
                }

                noise_map[x as usize][y as usize][z as usize] = val as f64;
            }
        }
    }

    return noise_map;
}

fn random_gradient(ix: i32, iy: i32, iz: i32) -> Vec3 {
    const W: u32 = 8 * std::mem::size_of::<u32>() as u32;
    const S: u32 = W / 2;
    let mut a = ix as u32;
    let mut b = iy as u32;
    let mut c = iz as u32;

    // Black magic from C, go see this reference:
    // https://www.pastebin.com/XwCPn0xR
    a = a.wrapping_mul(3284157443);
    b ^= a << S | a >> W - S;
    b = b.wrapping_mul(1911520717);
    a ^= b << S | b >> W - S;
    a = a.wrapping_mul(2048419325);

    c ^= b << S | b >> W - S;
    c = c.wrapping_mul(4294967197);

    a ^= c << S | c >> W - S;
    b ^= a << S | a >> W - S;

    a ^= b << S | b >> W - S;

    let random_vec = Vec3 {
        x: ((a % W) as f32) / (W as f32 / 2.0) - 1.0,
        y: ((b % W) as f32) / (W as f32 / 2.0) - 1.0,
        z: ((c % W) as f32) / (W as f32 / 2.0) - 1.0,
    };

    let len =
        (random_vec.x * random_vec.x + random_vec.y * random_vec.y + random_vec.z * random_vec.z)
            .sqrt();
    Vec3 {
        x: random_vec.x / len,
        y: random_vec.y / len,
        z: random_vec.z / len,
    }
}

fn dot_grid_gradient(ix: i32, iy: i32, iz: i32, x: f32, y: f32, z: f32) -> f32 {
    let gradient: Vec3 = random_gradient(ix, iy, iz);
    let dx: f32 = x - ix as f32;
    let dy: f32 = y - iy as f32;
    let dz: f32 = z - iz as f32;
    return dx * gradient.x + dy * gradient.y + dz * gradient.z;
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0;
}

fn perlin_3d(x: f32, y: f32, z: f32) -> f32 {
    let x0: i32 = x.floor() as i32;
    let x1: i32 = x0 + 1;
    let y0: i32 = y.floor() as i32;
    let y1: i32 = y0 + 1;
    let z0: i32 = z.floor() as i32;
    let z1: i32 = z0 + 1;

    let sx: f32 = x - x0 as f32;
    let sy: f32 = y - y0 as f32;
    let sz: f32 = z - z0 as f32;

    let n0: f32 = dot_grid_gradient(x0, y0, z0, x, y, z);
    let n1: f32 = dot_grid_gradient(x1, y0, z0, x, y, z);
    let ix0: f32 = interpolate(n0, n1, sx);

    let n2: f32 = dot_grid_gradient(x0, y1, z0, x, y, z);
    let n3: f32 = dot_grid_gradient(x1, y1, z0, x, y, z);
    let ix1: f32 = interpolate(n2, n3, sx);

    let iy0: f32 = interpolate(ix0, ix1, sy);

    let n4: f32 = dot_grid_gradient(x0, y0, z1, x, y, z);
    let n5: f32 = dot_grid_gradient(x1, y0, z1, x, y, z);
    let ix2: f32 = interpolate(n4, n5, sx);

    let n6: f32 = dot_grid_gradient(x0, y1, z1, x, y, z);
    let n7: f32 = dot_grid_gradient(x1, y1, z1, x, y, z);
    let ix3: f32 = interpolate(n6, n7, sx);

    let iy1: f32 = interpolate(ix2, ix3, sy);

    let val: f32 = interpolate(iy0, iy1, sz);
    return val;
}
