mod setup;
mod skybox;
mod camera;
mod planet;

use bevy::{
    prelude::*,
    pbr::wireframe::WireframePlugin
};
use bevy_mod_raycast::prelude::*;
use camera::ThirdPersonCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()),
            ThirdPersonCameraPlugin,
            DefaultRaycastingPlugin,
            WireframePlugin
        ))
        .add_systems(Startup, (skybox::build_skybox, setup::setup))
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
