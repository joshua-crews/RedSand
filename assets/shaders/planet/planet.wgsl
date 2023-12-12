#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(1) var material_color_texture: texture_2d<f32>;
@group(1) @binding(2) var material_color_sampler: sampler;
@group(1) @binding(3) var border_texture: texture_2d<f32>;
@group(1) @binding(4) var border_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let material_color: vec4<f32> = textureSample(material_color_texture, material_color_sampler, mesh.uv);
    let border_color: vec4<f32> = textureSample(border_texture, border_sampler, mesh.uv);

    let blended_color: vec4<f32> = mix(material_color, border_color, border_color.a);
    return blended_color;
}

