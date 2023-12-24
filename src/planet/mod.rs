use std::usize;

use bevy::{
    pbr::{
        wireframe::{Wireframe, WireframeColor},
        ExtendedMaterial,
    },
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use image::RgbImage;

use crate::{camera_system, game_assets};

mod noise;
mod planet_material;
mod planet_mesh;
mod provinces;

const NUM_PROVINCES: usize = 120;
pub const MAP_HEIGHT: u32 = 1080;
pub const MAP_WIDTH: u32 = 1920;

#[derive(Asset, AssetCollection, Resource, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub border_texture: Option<Handle<Image>>,
}

pub struct PlanetMesh {
    pub resolution: u32,
    pub size: f32,
    pub height_map: Image,
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Province {
    pub id: i32,
    pub color: [u8; 3],
}

#[derive(AssetCollection, Resource)]
pub struct MapImage {
    pub image: RgbImage,
}

pub fn setup(
    mut commands: Commands,
    mut planet_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    loaded_images: Res<Assets<Image>>,
    image_assets: Res<game_assets::ImageAssets>,
    normal_assets: Res<game_assets::NormalMapAssets>,
    height_assets: Res<game_assets::HeightMapAssets>,
    asset_server: Res<AssetServer>,
) {
    let colors = provinces::create_province_colors(NUM_PROVINCES, MAP_WIDTH, MAP_HEIGHT);
    for (province_id, color) in colors.iter().enumerate() {
        commands.spawn(Province {
            id: province_id as i32,
            color: color.0 .0,
        });
    }
    let provinces_map = provinces::create_provinces_image(colors, MAP_WIDTH, MAP_HEIGHT);
    commands.insert_resource(MapImage {
        image: provinces_map,
    });

    let directions = [
        Vec3::Y,
        Vec3::NEG_Y,
        Vec3::NEG_X,
        Vec3::X,
        Vec3::Z,
        Vec3::NEG_Z,
    ];
    for direction in directions {
        let planet_face = planet_mesh::spawn_face(direction, &loaded_images, &height_assets);
        if let Some(checked_face) = planet_face {
            let planet = (
                MaterialMeshBundle {
                    mesh: meshes.add(checked_face),
                    material: planet_mats.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color_texture: Some(image_assets.color_texture.clone()),
                            perceptual_roughness: 0.4,
                            normal_map_texture: Some(normal_assets.sample_normal.clone()),
                            ..Default::default()
                        },
                        extension: PlanetMaterial {
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
        }
    }
}
