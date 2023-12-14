use std::usize;

use bevy::{prelude::*, reflect::TypePath, render::render_resource::AsBindGroup};
use bevy_asset_loader::asset_collection::AssetCollection;
use image::RgbImage;

mod noise;
mod planet_material;
mod planet_mesh;
mod provinces;

const NUM_PROVINCES: usize = 120;
pub const MAP_HEIGHT: u32 = 1080;
pub const MAP_WIDTH: u32 = 1920;

#[derive(Asset, AssetCollection, Resource, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub border_texture: Option<Handle<Image>>,
}

pub struct PlanetMesh {
    pub resolution: u32,
    pub size: f32,
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

pub fn setup(mut commands: Commands) {
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
    })
}
