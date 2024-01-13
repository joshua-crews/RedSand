use bevy::{
    ecs::system::{CommandQueue, SystemState},
    prelude::*,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};

use image::{Rgb, RgbImage, RgbaImage};

use bevy::ecs::system::SystemParam;
use bevy_asset_loader::prelude::*;
use futures_lite::future;

use crate::camera_system;
use crate::game_assets;
use crate::planet;
use crate::setup;
use crate::skybox;

#[derive(Component)]
struct ComputeMeshesComponent(Task<(Vec<RgbImage>, Vec<Rgb<u8>>, Vec<RgbaImage>)>);

#[derive(Component)]
struct LoadingScreenComponent;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingImageAssets)
                    .continue_to_state(AppState::GeneratingMaps),
            )
            .add_collection_to_loading_state::<_, game_assets::ImageAssets>(
                AppState::LoadingImageAssets,
            )
            .add_collection_to_loading_state::<_, game_assets::ColorMapAssets>(
                AppState::LoadingImageAssets,
            )
            .add_collection_to_loading_state::<_, game_assets::NormalMapAssets>(
                AppState::LoadingImageAssets,
            )
            .add_collection_to_loading_state::<_, game_assets::HeightMapAssets>(
                AppState::LoadingImageAssets,
            )
            .add_systems(OnEnter(AppState::LoadingImageAssets), loading_screen)
            //.add_systems(OnEnter(AppState::GeneratingMeshes), generate_meshes_instructions)
            .add_systems(OnEnter(AppState::GeneratingMaps), setup_maps)
            .add_systems(
                Update,
                handle_map_generation_tasks.run_if(in_state(AppState::GeneratingMaps)),
            )
            .add_systems(
                OnEnter(AppState::SpawningMeshes),
                (
                    planet::setup,
                    skybox::build_skybox,
                    setup::setup,
                    finish_mesh_spawning,
                )
                    .chain(),
            )
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

fn setup_maps(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        let colors = planet::create_province_colors_async().await;
        let provinces_map = planet::create_province_images_async(colors).await;
        let province_data = planet::create_province_data_async(provinces_map.clone()).await;
        let border_data = planet::create_border_images_async(provinces_map.clone()).await;
        return (provinces_map, province_data, border_data);
    });

    commands.spawn(()).insert(ComputeMeshesComponent(task));
}

fn handle_map_generation_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeMeshesComponent)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for (entity, mut task_component) in tasks.iter_mut() {
        let future = future::block_on(future::poll_once(&mut task_component.0));
        if let Some((provinces_map, province_data, border_data)) = future {
            for (province_id, color) in province_data.iter().enumerate() {
                commands.spawn(planet::Province {
                    id: province_id as i32,
                    color: color.0,
                });
            }
            for province_map in &provinces_map {
                commands.insert_resource(planet::MapImage {
                    image: province_map.clone(),
                })
            }
            commands.insert_resource(planet::BorderImages {
                border_images: border_data,
            });
            commands.entity(entity).remove::<ComputeMeshesComponent>();
            state.set(AppState::SpawningMeshes);
        }
    }
}

fn finish_mesh_spawning(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingImageAssets,
    GeneratingMaps,
    SpawningMeshes,
    InGame,
}
