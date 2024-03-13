use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "textures/skybox/cubemap2.png")]
    pub skybox_texture: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ColorMapAssets {
    #[asset(path = "textures/surface/muddy_sand_albedo.png")]
    pub muddy_sand: Handle<Image>,
    #[asset(path = "textures/surface/volcanic_rock_albedo.png")]
    pub volcanic_rock: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct NormalMapAssets {
    #[asset(path = "textures/surface/muddy_sand_normal.png")]
    pub muddy_sand: Handle<Image>,
    #[asset(path = "textures/surface/volcanic_rock_normal.png")]
    pub volcanic_rock: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct HeightMapAssets {
    #[asset(path = "textures/mars/height/left.png")]
    pub negative_x: Handle<Image>,
    #[asset(path = "textures/mars/height/bottom.png")]
    pub negative_y: Handle<Image>,
    #[asset(path = "textures/mars/height/back.png")]
    pub negative_z: Handle<Image>,
    #[asset(path = "textures/mars/height/right.png")]
    pub positive_x: Handle<Image>,
    #[asset(path = "textures/mars/height/top.png")]
    pub positive_y: Handle<Image>,
    #[asset(path = "textures/mars/height/front.png")]
    pub positive_z: Handle<Image>,
}
