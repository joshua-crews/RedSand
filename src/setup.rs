use bevy::{
    pbr::wireframe::{Wireframe, WireframeColor},
    prelude::*,
};

use crate::camera_system;
use crate::game_assets::ImageAssets;
use crate::planet;
use crate::skybox;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planet_materials: ResMut<Assets<planet::PlanetMaterial>>,
    image_assets: Res<ImageAssets>,
) {
    let planet = (
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(planet::PlanetMesh {
                resolution: 20,
                size: 1.0,
            })),
            material: planet_materials.add(planet::PlanetMaterial {
                color_texture: Some(image_assets.color_texture.clone()),
                border_texture: Some(image_assets.border_texture.clone()),
            }),
            ..default()
        },
        /*
        Wireframe,
        WireframeColor {
            color: Color::BLACK,
        },
        */
        camera_system::ThirdPersonCameraTarget,
    );

    commands.spawn(planet);
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.insert_resource(skybox::Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: image_assets.skybox_texture.clone(),
    });
}
