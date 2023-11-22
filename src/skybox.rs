use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
        texture::CompressedImageFormats,
    },
};
use std::f32::consts::PI;

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/skybox/cubemap2.png",
        CompressedImageFormats::NONE,
    ),
];

#[derive(Resource)]
pub struct Cubemap {
    pub is_loaded: bool,
    pub index: usize,
    pub image_handle: Handle<Image>,
}

pub fn build_skybox(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..Default::default()
    });

    let skybox_handle = asset_server.load(CUBEMAPS[0].0);

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

pub const CUBEMAP_SWAP_DELAY: f32 = 3.0;

pub fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_seconds();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!("Skipping unsupported format: {:?}", CUBEMAPS[new_index]);
    }

    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}

pub fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}