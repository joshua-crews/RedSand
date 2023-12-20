mod camera_system;
mod game_assets;
mod loading_screen;
mod planet;
mod setup;
mod skybox;

use bevy::{
    pbr::{wireframe::WireframePlugin, ExtendedMaterial},
    prelude::*,
};
use bevy_mod_raycast::prelude::*;
use camera_system::ThirdPersonCameraPlugin;
use planet::PlanetMaterial;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(bevy_mod_raycast::low_latency_window_plugin())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1280.0, 720.0).into(),
                        title: "Red Sands".into(),
                        ..default()
                    }),
                    ..default()
                }),
            loading_screen::LoadingScreenPlugin,
            ThirdPersonCameraPlugin,
            DefaultRaycastingPlugin,
            WireframePlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, PlanetMaterial>>::default(),
        ))
        .run();
}
