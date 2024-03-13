// The following code is an adaptation from:
// https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
fn permute(x: vec4<f32>) -> vec4<f32> {
    return fract((x * 34.0 + vec4<f32>(1.0)) * x) % vec4<f32>(289.0);
}

fn fade(t: vec2<f32>) -> vec2<f32> {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

fn perlinNoise2(original_uv: vec2<f32>, offset: vec2<f32>, scale: vec2<f32>) -> f32 {
    let P = (original_uv * scale) + offset;
    var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0.0, 0.0, 1.0, 1.0);
    let Pf = fract(P.xyxy) - vec4<f32>(0.0, 0.0, 1.0, 1.0);
    Pi = Pi % vec4<f32>(289.0);
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute(permute(ix) + iy);
    var gx: vec4<f32> = 2.0 * fract(i * 0.0243902439) - 1.0;
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
    var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
    var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
    var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
    let norm = taylorInvSqrt(
        vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11))
    );
    g00 *= norm.x;
    g01 *= norm.y;
    g10 *= norm.z;
    g11 *= norm.w;
    let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
    let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
    let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
    let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
    let fade_xy = fade(Pf.xy);
    let n_x = mix(vec2<f32>(n00, n10), vec2<f32>(n01, n11), fade_xy.x);
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy; // Shading factor, bigger number means more sharp the boundary between biomes is
}

fn taylorInvSqrt(r: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - 0.85373472095314 * r;
}

fn calculate_biome_value(uv: vec2<f32>) -> f32 {
    //TODO: Figure out scaling the UV vector by to ensure a more proper visual effect and use via engine config or biomes config perhaps
    let uv_offset = vec2<f32>(100.0, 2.0);
    let uv_scale = vec2<f32>(1.0, 1.0);
    let noise = perlinNoise2(uv, uv_offset, uv_scale);
    return smoothstep(0.3, 0.7, noise);
}
