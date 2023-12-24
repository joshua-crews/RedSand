use bevy::{
    ecs::system::{CommandQueue, SystemState},
    prelude::*,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};

use bevy_asset_loader::prelude::*;

use crate::camera_system;
use crate::game_assets;
use crate::planet;
use crate::setup;
use crate::skybox;

#[derive(Component)]
struct ComputeMeshesComponent(Task<()>);

#[derive(Component)]
struct LoadingScreenComponent;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        let generate_meshes_instructions =
            (planet::setup, skybox::build_skybox, setup::setup).chain();
        app.add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingImageAssets)
                    .continue_to_state(AppState::InGame),
            )
            .add_collection_to_loading_state::<_, game_assets::ImageAssets>(
                AppState::LoadingImageAssets,
            )
            .add_collection_to_loading_state::<_, game_assets::NormalMapAssets>(
                AppState::LoadingImageAssets,
            )
            .add_collection_to_loading_state::<_, game_assets::HeightMapAssets>(
                AppState::LoadingImageAssets,
            )
            .add_systems(OnEnter(AppState::LoadingImageAssets), loading_screen)
            .add_systems(OnTransition{from: AppState::LoadingImageAssets, to: AppState::InGame}, generate_meshes_instructions) /* This is crazy slow and should be async awaited to not cause a hang in the program */
            .add_systems(OnEnter(AppState::InGame), enter_game)
            .add_systems(
                Update,
                (
                    skybox::asset_loaded.run_if(in_state(AppState::InGame)),
                    close_on_esc.run_if(in_state(AppState::InGame)),
                ),
            );
    }
}

fn enter_game(
    mut commands: Commands,
    loading_query: Query<Entity, With<LoadingScreenComponent>>,
    skybox_cubemap: Res<game_assets::ImageAssets>,
) {
    for loading_component in loading_query.iter() {
        commands.entity(loading_component).despawn();
    }
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        camera_system::ThirdPersonCamera::default(),
        bevy::core_pipeline::Skybox(skybox_cubemap.skybox_texture.clone()),
    );
    commands.spawn(camera);
}

fn loading_screen(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), LoadingScreenComponent));
    commands.spawn((
        TextBundle::from_section(
            "Loading...",
            TextStyle {
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center),
        LoadingScreenComponent,
    ));
}

fn close_on_esc(
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

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    LoadingImageAssets,
    GeneratingMeshes,
    InGame,
}
