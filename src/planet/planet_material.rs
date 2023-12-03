use bevy::{
    prelude::*,
    render::{
        render_resource::{
            ShaderRef,
        },
    },
};
use crate::planet;

impl Material for planet::PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/planet.wgsl".into()
    }
}