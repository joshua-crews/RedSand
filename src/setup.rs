use bevy::{
    asset::AssetLoader,
    pbr::wireframe::{Wireframe, WireframeColor},
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};

use crate::camera_system;
use crate::game_assets;
use crate::planet;
use crate::skybox;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_materials: ResMut<Assets<planet::PlanetMaterial>>,
    image_assets: Res<game_assets::ImageAssets>,
    normal_assets: Res<game_assets::NormalMapAssets>,
    asset_server: Res<AssetServer>,
) {
    let planet = (
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(planet::PlanetMesh {
                resolution: 6,
                size: 1.0,
            })),
            material: planet_materials.add(planet::PlanetMaterial {
                color_texture: Some(image_assets.color_texture.clone()),
                border_texture: Some(asset_server.load("saves/borders.png")),
                normal_map: Some(normal_assets.sample_normal.clone()),
            }),
            ..default()
        },
        Wireframe,
        WireframeColor {
            color: Color::BLACK,
        },
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
