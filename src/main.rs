mod camera;
mod planet;
mod setup;
mod skybox;

use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_mod_raycast::prelude::*;
use camera::ThirdPersonCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            /*
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(),
                    title: "Red Sands".to_string(),
                    ..default()
                }),
                ..default()
            }),
            */
            DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()),
            ThirdPersonCameraPlugin,
            DefaultRaycastingPlugin,
            WireframePlugin,
            MaterialPlugin::<planet::PlanetMaterial>::default(),
        ))
        .add_systems(
            Startup,
            (planet::setup, skybox::build_skybox, setup::setup).chain(),
        )
        .add_systems(
            Update,
            (
                skybox::cycle_cubemap_asset,
                skybox::asset_loaded.after(skybox::cycle_cubemap_asset),
                close_on_esc,
            ),
        )
        .run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<Input<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
