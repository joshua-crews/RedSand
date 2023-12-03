use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy::ecs::query::QuerySingleError;
use bevy_mod_raycast::prelude::*;

use crate::{camera};
use crate::camera::ThirdPersonCamera;


#[derive(Resource)]
pub struct CursorOverPlanet(bool);

#[derive(Resource)]
pub struct CamVelocity(Vec2);
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorOverPlanet(false))
            .insert_resource(CamVelocity(Vec2::ZERO))
            .add_systems(
            Update,
            (
                ray_cast_planet,
                orbit_mouse,
                zoom_mouse.run_if(camera::zoom_condition),
            )
            .chain(),
        );
    }
}

fn ray_cast_planet(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    planet_q: Query<&Transform, With<camera::ThirdPersonCameraTarget>>,
    mut found_planet: ResMut<CursorOverPlanet>,
) {
    let mut still_over: bool = false;
    if let Some(cursor_ray) = **cursor_ray {
        let val: &[(Entity, IntersectionData)] = raycast.cast_ray(cursor_ray, &default());
        for (entity, _intersection_data) in val.iter() {
            if planet_q.get(*entity).is_ok() {
                found_planet.0 = true;
                still_over = true;
                break;
            }
        }
        if !still_over {
            found_planet.0 = false;
        }
    }
}

fn orbit_condition(cam: &ThirdPersonCamera,
                   mouse: &Res<Input<MouseButton>>,
                   found_planet: &Res<CursorOverPlanet>,
) -> bool {
    if mouse.pressed(cam.mouse_orbit_button) && found_planet.0 {
        return true;
    }
    return false;
}

pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&ThirdPersonCamera, &mut Transform), With<ThirdPersonCamera>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_evr: EventReader<MouseMotion>,
    mut cam_velocity: ResMut<CamVelocity>,
    found_planet: Res<CursorOverPlanet>,
) {
    let rotation: Vec2;
    let Ok((cam, mut cam_transform)):
        Result<(&ThirdPersonCamera, Mut<Transform>), QuerySingleError> =
        cam_q.get_single_mut() else { return };
    let mut position: Vec2 = Vec2::new(0.0, 0.0);
    for ev in mouse_evr.read() {
        if orbit_condition(cam, &mouse, &found_planet) {
            cam_velocity.0 = ev.delta * cam.mouse_sensitivity;
        }
        position = ev.delta * cam.mouse_sensitivity;
    }

    if !orbit_condition(cam, &mouse, &found_planet) {
        rotation = cam_velocity.0;
        cam_velocity.0 *= cam.inertia;
    } else {
        rotation = position;
        cam_velocity.0 = position;
    }

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta: f32 = rotation.x / window.width() * PI;
            delta
        };

        let delta_y: f32 = rotation.y / window.height() * PI;
        let yaw: Quat = Quat::from_rotation_y(-delta_x);
        let pitch: Quat = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation;

        let new_rotation: Quat = cam_transform.rotation * pitch;
        let up_vector: Vec3 = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix: Mat3 = Mat3::from_quat(cam_transform.rotation);
    cam_transform.translation =
        cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.zoom.radius));
}

fn zoom_mouse(mut scroll_evr: EventReader<MouseWheel>, mut cam_q: Query<&mut ThirdPersonCamera>) {
    let mut scroll: f32 = 0.0;
    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    if let Ok(mut cam) = cam_q.get_single_mut() {
        if scroll.abs() > 0.0 {
            let new_radius: f32 =
                cam.zoom.radius - scroll * cam.zoom.radius * 0.1 * cam.zoom_sensitivity;
            cam.zoom.radius = new_radius.clamp(cam.zoom.min, cam.zoom.max);
        }
    }
}