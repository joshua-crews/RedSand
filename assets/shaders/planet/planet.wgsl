#import bevy_pbr::{
  pbr_fragment::pbr_input_from_standard_material,
  pbr_functions::alpha_discard,
}

#import "shaders/planet/biomes.wgsl"::calculate_biome_value

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
  prepass_io::{VertexOutput, FragmentOutput},
  pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
  forward_io::{VertexOutput, FragmentOutput},
  pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(1) @binding(100) 
var<uniform> repeat_factor: f32;
@group(1) @binding(101)
var border_texture: texture_2d<f32>;
@group(1) @binding(102)
var border_sampler: sampler;
@group(1) @binding(103)
var base_texture: texture_2d<f32>;
@group(1) @binding(104)
var base_sampler: sampler;
@group(1) @binding(105)
var rock_texture: texture_2d<f32>;
@group(1) @binding(106)
var rock_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    
    let tiled_uv = vec2<f32>(
      fract(in.uv.x * repeat_factor),
      fract(in.uv.y * repeat_factor)
    );

    let single_uv = vec2<f32>(
      fract(in.uv.x / repeat_factor),
      fract(in.uv.y / repeat_factor)
    );

    let base_texture_tiled: vec4<f32> = textureSample(base_texture, base_sampler, tiled_uv);
    let rock_texture_tiled: vec4<f32> = textureSample(rock_texture, rock_sampler, tiled_uv);

    let biome_value: f32 = calculate_biome_value(single_uv);
    let mixed_rock: vec4<f32> = mix(base_texture_tiled, rock_texture_tiled, biome_value);

    let mixed_terrain_color: vec4<f32> = mix(pbr_input.material.base_color, mixed_rock, mixed_rock.a);
    let border_color: vec4<f32> = textureSample(border_texture, border_sampler, single_uv);
    let mixed_terrain: vec4<f32> = mix(mixed_terrain_color, border_color, border_color.a);
    pbr_input.material.base_color = mixed_terrain;
#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif
    return mixed_terrain;
}

