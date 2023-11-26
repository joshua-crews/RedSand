use bevy::prelude::*;
use bevy::pbr::wireframe::{Wireframe, WireframeColor};

use crate::camera;
use crate::skybox;
use crate::planet;

#[derive(Component)]
pub struct Ground;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let planet = (
        PbrBundle {
            mesh: meshes.add(planet::PlanetShape::default().into()),
            material: materials.add(Color::SILVER.into()),
            ..Default::default()
        },
        Wireframe,
        WireframeColor {
            color: Color::BLACK,
        },
        camera::ThirdPersonCameraTarget,
    );

    commands.spawn(planet);
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let skybox_handle = asset_server.load(skybox::CUBEMAPS[0].0);
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera::ThirdPersonCamera::default(),
        bevy::core_pipeline::Skybox(skybox_handle.clone()),
    );
    commands.spawn(camera);

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.insert_resource(skybox::Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}