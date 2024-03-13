use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};

use image::{Rgb, RgbImage, RgbaImage};

use bevy_asset_loader::prelude::*;
use futures_lite::future;

use crate::camera_system;
use crate::config_parser;
use crate::game_assets;
use crate::planet;
use crate::setup;
use crate::skybox;

#[derive(Component)]
struct ComputeMapsComponent(Task<(Vec<RgbImage>, Vec<Rgb<u8>>, Vec<RgbaImage>)>);

#[derive(Component)]
struct ComputeMeshesComponent(Task<(Vec3, String, Vec<Mesh>)>);

#[derive(Component)]
struct LoadingScreenComponent;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::LoadingConfigs)
                    .continue_to_state(AppState::LoadingImageAssets),
            )
            .add_systems(
                OnEnter(AppState::LoadingConfigs),
                config_parser::read_configs,
            )
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
            .add_systems(OnEnter(AppState::GeneratingMaps), setup_maps)
            .add_systems(
                Update,
                handle_map_generation_tasks.run_if(in_state(AppState::GeneratingMaps)),
            )
            .add_systems(OnEnter(AppState::GeneratingMeshes), setup_meshes)
            .add_systems(
                Update,
                handle_mesh_generation_tasks.run_if(in_state(AppState::GeneratingMeshes)),
            )
            .add_systems(
                OnEnter(AppState::SpawningGameEntities),
                (
                    planet::setup,
                    skybox::build_skybox,
                    setup::setup,
                    finish_entity_spawning,
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

fn setup_meshes(
    mut commands: Commands,
    height_assets: Res<game_assets::HeightMapAssets>,
    loaded_images: Res<Assets<Image>>,
    engine_config: Res<config_parser::EngineConfig>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    commands.insert_resource(planet::PlanetLODs {
        level_of_detail_meshes: Vec::new(),
    });
    let directions = [
        (Vec3::Y, "positive_y"),
        (Vec3::NEG_Y, "negative_y"),
        (Vec3::NEG_X, "negative_x"),
        (Vec3::X, "positive_x"),
        (Vec3::Z, "positive_z"),
        (Vec3::NEG_Z, "negative_z"),
    ];

    for (direction, suffix) in directions {
        let height_handle = match suffix {
            "positive_y" => &height_assets.positive_y,
            "negative_y" => &height_assets.negative_y,
            "negative_x" => &height_assets.negative_x,
            "positive_x" => &height_assets.positive_x,
            "positive_z" => &height_assets.positive_z,
            "negative_z" => &height_assets.negative_z,
            _ => continue,
        };
        let height_handle_clone: Handle<Image> = height_handle.clone();
        let height_map_clone: Image = loaded_images.get(height_handle_clone).unwrap().clone();
        let planet_lods: Vec<u32> = engine_config.planet_lods.clone();
        let size: f32 = engine_config.planet_scale as f32;
        let uv_scale: f32 = engine_config.uv_scale as f32;

        let task = thread_pool.spawn(async move {
            let mut faces: Vec<Mesh> = Vec::with_capacity(planet_lods.len() as usize);
            for res in planet_lods {
                let planet_face =
                    planet::spawn_face(res, size, direction, uv_scale, &height_map_clone);
                faces.push(planet_face);
            }
            return (direction, suffix.to_owned(), faces);
        });

        commands.spawn(()).insert(ComputeMeshesComponent(task));
    }
}

fn handle_mesh_generation_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeMeshesComponent)>,
    mut state: ResMut<NextState<AppState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planet_lods: ResMut<planet::PlanetLODs>,
) {
    for (entity, mut task_component) in tasks.iter_mut() {
        let future = future::block_on(future::poll_once(&mut task_component.0));
        if let Some((direction, suffix, planet_lods_computed)) = future {
            let mut handle_faces: Vec<Handle<Mesh>> =
                Vec::with_capacity(planet_lods_computed.len());
            for face in planet_lods_computed {
                handle_faces.push(meshes.add(face));
            }
            planet_lods
                .level_of_detail_meshes
                .push((direction, suffix, handle_faces));
            commands.entity(entity).despawn();
        }
    }
    if tasks.iter().count() <= 0 {
        info!(target: "red_sand::loading_state::systems", "Loading state 'red_sand::loading_screen::AppState::GeneratingMeshes' is done");
        state.set(AppState::SpawningGameEntities);
    }
}

fn setup_maps(mut commands: Commands, engine_config: Res<config_parser::EngineConfig>) {
    let thread_pool = AsyncComputeTaskPool::get();
    let num_provinces: u32 = engine_config.num_provinces;
    let map_dimensions: u32 = engine_config.map_dimensions;
    let task = thread_pool.spawn(async move {
        let colors = planet::create_province_colors_async(num_provinces, map_dimensions).await;
        let provinces_map = planet::create_province_images_async(colors, map_dimensions).await;
        let province_data = planet::create_province_data_async(provinces_map.clone()).await;
        let border_data =
            planet::create_border_images_async(provinces_map.clone(), map_dimensions).await;
        return (provinces_map, province_data, border_data);
    });

    commands.spawn(()).insert(ComputeMapsComponent(task));
}

fn handle_map_generation_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ComputeMapsComponent)>,
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
            commands.entity(entity).remove::<ComputeMapsComponent>();
            info!(target: "red_sand::loading_state::systems", "Loading state 'red_sand::loading_screen::AppState::GeneratingMaps' is done");
            state.set(AppState::GeneratingMeshes);
        }
    }
}

fn finish_entity_spawning(mut state: ResMut<NextState<AppState>>) {
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
    LoadingConfigs,
    LoadingImageAssets,
    GeneratingMaps,
    GeneratingMeshes,
    SpawningGameEntities,
    InGame,
}
