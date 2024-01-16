use bevy::prelude::*;

use crate::game_assets;
use crate::skybox;

pub fn setup(mut commands: Commands, image_assets: Res<game_assets::ImageAssets>) {
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
