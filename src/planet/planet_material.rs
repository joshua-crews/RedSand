use crate::planet;
use bevy::{pbr::MaterialExtension, render::render_resource::ShaderRef};

impl MaterialExtension for planet::PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/planet.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/planet/planet.wgsl".into()
    }
}
