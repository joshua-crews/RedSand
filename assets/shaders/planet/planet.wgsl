#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(1) var material_color_texture: texture_2d<f32>;
@group(1) @binding(2) var material_color_sampler: sampler;
@group(1) @binding(3) var border_texture: texture_2d<f32>;
@group(1) @binding(4) var border_sampler: sampler;
@group(1) @binding(5) var normal_texture: texture_2d<f32>;
@group(1) @binding(6) var normal_sampler: sampler;

// Mock-up constants for directional light parameters for now cause I am lazy af
const light_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
const ambient_color: vec3<f32> = vec3<f32>(0.1, 0.1, 0.1);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Wanna make this a const but dunno how since normalize aint constant
    let light_direction: vec3<f32> = normalize(vec3<f32>(0.3, -1.0, -0.3));

    let material_color: vec4<f32> = textureSample(material_color_texture, material_color_sampler, mesh.uv);
    let border_color: vec4<f32> = textureSample(border_texture, border_sampler, mesh.uv);
    let blended_color: vec4<f32> = mix(material_color, border_color, border_color.a);

    var normal_tex_color: vec4<f32> = textureSample(normal_texture, normal_sampler, mesh.uv);

    var normal_vector: vec3<f32> = normalize(normal_tex_color.rgb * 2.0 - 1.0);

    let light_effect: f32 = max(dot(normal_vector, light_direction), 0.0);
    let diffused_light: vec3<f32> = light_effect * light_color;

    let lighting: vec3<f32> = diffused_light + ambient_color;
    let color_with_lighting: vec3<f32> = blended_color.rgb * lighting;

    return vec4<f32>(color_with_lighting, blended_color.a);
}

