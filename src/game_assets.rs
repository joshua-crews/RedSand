use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "textures/8k_mars.png")]
    pub color_texture: Handle<Image>,
    #[asset(path = "saves/borders.png")]
    pub border_texture: Handle<Image>,
    #[asset(path = "textures/skybox/cubemap2.png")]
    pub skybox_texture: Handle<Image>,
}