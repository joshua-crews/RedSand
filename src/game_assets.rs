use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "textures/skybox/cubemap2.png")]
    pub skybox_texture: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ColorMapAssets {
    #[asset(path = "textures/mars/color/left.png")]
    pub negative_x: Handle<Image>,
    #[asset(path = "textures/mars/color/bottom.png")]
    pub negative_y: Handle<Image>,
    #[asset(path = "textures/mars/color/back.png")]
    pub negative_z: Handle<Image>,
    #[asset(path = "textures/mars/color/right.png")]
    pub positive_x: Handle<Image>,
    #[asset(path = "textures/mars/color/top.png")]
    pub positive_y: Handle<Image>,
    #[asset(path = "textures/mars/color/front.png")]
    pub positive_z: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct NormalMapAssets {
    #[asset(path = "textures/mars/normal/left.png")]
    pub negative_x: Handle<Image>,
    #[asset(path = "textures/mars/normal/bottom.png")]
    pub negative_y: Handle<Image>,
    #[asset(path = "textures/mars/normal/back.png")]
    pub negative_z: Handle<Image>,
    #[asset(path = "textures/mars/normal/right.png")]
    pub positive_x: Handle<Image>,
    #[asset(path = "textures/mars/normal/top.png")]
    pub positive_y: Handle<Image>,
    #[asset(path = "textures/mars/normal/front.png")]
    pub positive_z: Handle<Image>,
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
