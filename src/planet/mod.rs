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
use image::{DynamicImage, RgbImage};

use crate::{camera_system, game_assets};

mod noise;
mod planet_material;
mod planet_mesh;
mod provinces;

const NUM_PROVINCES: usize = 120;
pub const MAP_DIMENSIONS: u32 = 300;

#[derive(Asset, AssetCollection, Resource, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[texture(100)]
    #[sampler(101)]
    pub border_texture: Option<Handle<Image>>,
}

pub struct PlanetMesh {
    pub resolution: u32,
    pub size: f32,
    pub direction: Vec3,
    pub height_map: Image,
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
    color_assets: Res<game_assets::ColorMapAssets>,
    normal_assets: Res<game_assets::NormalMapAssets>,
    height_assets: Res<game_assets::HeightMapAssets>,
    asset_server: Res<AssetServer>,
) {
    let colors = provinces::create_province_colors(NUM_PROVINCES, MAP_DIMENSIONS);
    let provinces_map = provinces::create_provinces_images(colors, MAP_DIMENSIONS);
    let province_data = provinces::get_colors(&provinces_map);
    for (province_id, color) in province_data.iter().enumerate() {
        commands.spawn(Province {
            id: province_id as i32,
            color: color.0,
        });
    }
    for province_map in &provinces_map {
        commands.insert_resource(MapImage {
            image: province_map.clone(),
        })
    }
    let border_images = provinces::get_border_images(MAP_DIMENSIONS, &provinces_map);

    let directions = [
        (Vec3::Y, "positive_y"),
        (Vec3::NEG_Y, "negative_y"),
        (Vec3::NEG_X, "negative_x"),
        (Vec3::X, "positive_x"),
        (Vec3::Z, "positive_z"),
        (Vec3::NEG_Z, "negative_z"),
    ];

    for (direction, suffix) in directions {
        let height_handle = match suffix {
            "positive_y" => &height_assets.positive_y,
            "negative_y" => &height_assets.negative_y,
            "negative_x" => &height_assets.negative_x,
            "positive_x" => &height_assets.positive_x,
            "positive_z" => &height_assets.positive_z,
            "negative_z" => &height_assets.negative_z,
            _ => continue,
        };
        let color_handle = match suffix {
            "positive_y" => color_assets.positive_y.clone(),
            "negative_y" => color_assets.negative_y.clone(),
            "negative_x" => color_assets.negative_x.clone(),
            "positive_x" => color_assets.positive_x.clone(),
            "positive_z" => color_assets.positive_z.clone(),
            "negative_z" => color_assets.negative_z.clone(),
            _ => continue,
        };

        let normal_handle = match suffix {
            "positive_y" => normal_assets.positive_y.clone(),
            "negative_y" => normal_assets.negative_y.clone(),
            "negative_x" => normal_assets.negative_x.clone(),
            "positive_x" => normal_assets.positive_x.clone(),
            "positive_z" => normal_assets.positive_z.clone(),
            "negative_z" => normal_assets.negative_z.clone(),
            _ => continue,
        };

        let border_image = match suffix {
            "positive_y" => border_images[5].clone(),
            "negative_y" => border_images[4].clone(),
            "negative_x" => border_images[1].clone(),
            "positive_x" => border_images[0].clone(),
            "positive_z" => border_images[3].clone(),
            "negative_z" => border_images[2].clone(),
            _ => continue,
        };

        let converted_border_image = bevy::render::texture::Image::from_dynamic(
            DynamicImage::ImageRgba8(border_image.into()),
            false,
        );
        let height_map = loaded_images.get(height_handle).unwrap();

        let planet_face = planet_mesh::spawn_face(direction, height_map);
        let planet = (
            MaterialMeshBundle {
                mesh: meshes.add(planet_face),
                material: planet_mats.add(ExtendedMaterial {
                    base: StandardMaterial {
                        base_color_texture: Some(color_handle),
                        perceptual_roughness: 0.4,
                        normal_map_texture: Some(normal_handle),
                        ..Default::default()
                    },
                    extension: PlanetMaterial {
                        border_texture: Some(asset_server.add(converted_border_image)),
                    },
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
    }
}
