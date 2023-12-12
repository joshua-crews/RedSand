use bevy::{core_pipeline::Skybox, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::camera_system;
use crate::game_assets::ImageAssets;
use crate::planet;
use crate::setup;
use crate::skybox;

#[derive(Component)]
struct LoadingScreenComponent;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hex("010d13").unwrap()))
            .add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::BootingApp).continue_to_state(AppState::TitleScreen),
            )
            .add_collection_to_loading_state::<_, ImageAssets>(AppState::BootingApp)
            .add_systems(OnEnter(AppState::BootingApp), loading_screen)
            .add_systems(
                OnEnter(AppState::TitleScreen),
                (
                    enter_game,
                    planet::setup,
                    skybox::build_skybox,
                    setup::setup,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    skybox::asset_loaded.run_if(in_state(AppState::TitleScreen)),
                    close_on_esc.run_if(in_state(AppState::TitleScreen)),
                ),
            );
    }
}

fn enter_game(
    mut commands: Commands,
    loading_query: Query<Entity, With<LoadingScreenComponent>>,
    skybox_cubemap: Res<ImageAssets>,
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
    BootingApp,
    TitleScreen,
    ErrorScreen,
}
