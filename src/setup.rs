use bevy::{
    pbr::{
        wireframe::{Wireframe, WireframeColor},
        ExtendedMaterial,
    },
    prelude::*,
    render::mesh::Mesh,
};

use crate::camera_system;
use crate::game_assets;
use crate::planet;
use crate::skybox;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planet_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, planet::PlanetMaterial>>>,
    loaded_images: Res<Assets<Image>>,
    image_assets: Res<game_assets::ImageAssets>,
    normal_assets: Res<game_assets::NormalMapAssets>,
    height_assets: Res<game_assets::HeightMapAssets>,
    asset_server: Res<AssetServer>,
) {
    if let Some(height_map_image) = loaded_images.get(&height_assets.sample_height) {
        let planet = (
            MaterialMeshBundle {
                mesh: meshes.add(
                    Mesh::from(planet::PlanetMesh {
                        resolution: 50,
                        size: 1.0,
                        height_map: height_map_image.clone(),
                    })
                    .with_generated_tangents()
                    .unwrap(),
                ),
                material: planet_mats.add(ExtendedMaterial {
                    base: StandardMaterial {
                        base_color_texture: Some(image_assets.color_texture.clone()),
                        perceptual_roughness: 0.4,
                        normal_map_texture: Some(normal_assets.sample_normal.clone()),
                        ..Default::default()
                    },
                    extension: planet::PlanetMaterial {
                        border_texture: Some(asset_server.load("saves/borders.png")),
                    },
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
    } else {
        println!("Error height map...");
    }
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
