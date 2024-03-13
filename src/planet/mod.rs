use std::usize;

#[allow(unused_imports)]
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
use image::{DynamicImage, Rgb, RgbImage, RgbaImage};

use crate::{camera_system, config_parser, game_assets};

mod noise;
mod planet_material;
mod planet_mesh;
mod provinces;

#[derive(Asset, AssetCollection, Resource, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(100)]
    pub uv_scale: f32,
    #[texture(101)]
    #[sampler(102)]
    pub border_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    pub base_texture: Option<Handle<Image>>,
    #[texture(105)]
    #[sampler(106)]
    pub rock_texture: Option<Handle<Image>>,
}

#[derive(Resource, Debug)]
pub struct BorderImages {
    pub border_images: Vec<RgbaImage>,
}

#[derive(Component)]
pub struct PlanetEntity {
    pub direction: String,
}

pub struct PlanetMesh {
    resolution: u32,
    size: f32,
    direction: Vec3,
    uv_scale: f32,
    height_map: Image,
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

#[derive(Resource)]
pub struct PlanetLODs {
    pub level_of_detail_meshes: Vec<(Vec3, String, Vec<Handle<Mesh>>)>,
}

pub async fn create_province_colors_async(
    num_provinces: u32,
    map_dimensions: u32,
) -> Vec<(Rgb<u8>, u32, u32, u32)> {
    return provinces::create_province_colors(num_provinces as usize, map_dimensions);
}

pub async fn create_province_images_async(
    colors: Vec<(Rgb<u8>, u32, u32, u32)>,
    map_dimensions: u32,
) -> Vec<RgbImage> {
    return provinces::create_provinces_images(colors, map_dimensions);
}

pub async fn create_province_data_async(province_map: Vec<RgbImage>) -> Vec<Rgb<u8>> {
    return provinces::get_colors(&province_map);
}

pub async fn create_border_images_async(
    provinces_map: Vec<RgbImage>,
    map_dimensions: u32,
) -> Vec<RgbaImage> {
    return provinces::get_border_images(map_dimensions, &provinces_map);
}

pub fn setup(
    mut commands: Commands,
    mut planet_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
    planet_lods: Res<PlanetLODs>,
    border_images: Res<BorderImages>,
    color_assets: Res<game_assets::ColorMapAssets>,
    normal_assets: Res<game_assets::NormalMapAssets>,
    asset_server: Res<AssetServer>,
    engine_config: Res<config_parser::EngineConfig>,
) {
    let directions = [
        (Vec3::Y, "positive_y"),
        (Vec3::NEG_Y, "negative_y"),
        (Vec3::NEG_X, "negative_x"),
        (Vec3::X, "positive_x"),
        (Vec3::Z, "positive_z"),
        (Vec3::NEG_Z, "negative_z"),
    ];

    for (_direction, suffix) in directions {
        let color_handle = color_assets.muddy_sand.clone();
        let rock_texture = color_assets.volcanic_rock.clone();
        let normal_handle = normal_assets.muddy_sand.clone();

        let border_image = match suffix {
            "positive_y" => border_images.border_images[5].clone(),
            "negative_y" => border_images.border_images[4].clone(),
            "negative_x" => border_images.border_images[1].clone(),
            "positive_x" => border_images.border_images[0].clone(),
            "positive_z" => border_images.border_images[3].clone(),
            "negative_z" => border_images.border_images[2].clone(),
            _ => continue,
        };

        let converted_border_image = bevy::render::texture::Image::from_dynamic(
            DynamicImage::ImageRgba8(border_image.into()),
            false,
        );
        let mut lod: Option<Handle<Mesh>> = None;
        let mut dir: String = "".to_owned();
        for (_direction, suffix_dir, lods) in planet_lods.level_of_detail_meshes.iter() {
            if suffix_dir == suffix {
                lod = Some(lods[0].clone());
                dir = suffix_dir.to_owned();
                break;
            }
        }

        if let Some(pulled_lod) = lod {
            let planet = (
                MaterialMeshBundle {
                    mesh: pulled_lod,
                    material: planet_mats.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::ORANGE_RED,
                            perceptual_roughness: 0.4,
                            normal_map_texture: Some(normal_handle),
                            ..Default::default()
                        },
                        extension: PlanetMaterial {
                            uv_scale: engine_config.uv_scale as f32,
                            border_texture: Some(asset_server.add(converted_border_image)),
                            base_texture: Some(color_handle),
                            rock_texture: Some(rock_texture),
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
                PlanetEntity { direction: dir },
            );
            commands.spawn(planet);
        }
    }
}

pub fn spawn_face(
    resolution: u32,
    size: f32,
    direction: Vec3,
    uv_scale: f32,
    height_map: &Image,
) -> Mesh {
    return planet_mesh::spawn_face(resolution, size, direction, uv_scale, height_map);
}
