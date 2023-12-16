use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "textures/8k_mars.png")]
    pub color_texture: Handle<Image>,
    #[asset(path = "textures/skybox/cubemap2.png")]
    pub skybox_texture: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct NormalMapAssets {
    #[asset(path = "textures/terrain_normals/sample_map.png")]
    pub sample_normal: Handle<Image>,
}
