use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::game::settings::KeyBinds;

const PLAYER_SPEED: f32 = 10.;
const PLAYER_SENSITIVITY: f32 = 0.0002;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>().add_systems(
            Update,
            (player_move, grab_mouse, player_look).run_if(in_state(crate::GameState::Playing)),
        );
    }
}
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Component)]
pub struct PlayerCam;

fn player_move(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    key_binds: Res<KeyBinds>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &PlayerCam)>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut transform, _camera) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    if key_binds.is_pressed("move_forward", &keys, &mouse) {
                        velocity += forward;
                    } else if key_binds.is_pressed("move_backward", &keys, &mouse) {
                        velocity -= forward;
                    } else if key_binds.is_pressed("move_left", &keys, &mouse) {
                        velocity -= right;
                    } else if key_binds.is_pressed("move_right", &keys, &mouse) {
                        velocity += right;
                    } else if key_binds.is_pressed("move_ascent", &keys, &mouse) {
                        velocity += Vec3::Y;
                    } else if key_binds.is_pressed("move_descent", &keys, &mouse) {
                        velocity -= Vec3::Y;
                    }
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * PLAYER_SPEED;
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn grab_mouse(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    key_binds: Res<KeyBinds>,
    mouse: Res<Input<MouseButton>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if let CursorGrabMode::None = window.cursor.grab_mode.clone() {
            if key_binds.is_pressed("grab_cursor", &keys, &mouse) {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
        } else {
            if key_binds.is_pressed("ungrab_cursor", &keys, &mouse) {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            } else {
                let w = window.physical_width() as f32;
                let h = window.physical_height() as f32;
                window.set_cursor_position(Some(Vec2::new(w / 2., h / 2.)));
            }
        }
    } else {
        warn!("Not found primary window!");
    }
}

fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<PlayerCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (PLAYER_SENSITIVITY * ev.delta.y * window_scale).to_radians();
                        yaw -= (PLAYER_SENSITIVITY * ev.delta.x * window_scale).to_radians();
                    }
                }
                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
