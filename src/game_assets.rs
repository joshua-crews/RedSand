use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "textures/mars/color.png")]
    pub color_texture: Handle<Image>,
    #[asset(path = "textures/skybox/cubemap2.png")]
    pub skybox_texture: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct NormalMapAssets {
    #[asset(path = "textures/mars/normal.png")]
    pub sample_normal: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct HeightMapAssets {
    #[asset(path = "textures/mars/height.png")]
    pub sample_height: Handle<Image>,
}
