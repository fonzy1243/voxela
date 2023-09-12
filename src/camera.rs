use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*, window::*};

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.05,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::ShiftLeft,
            walk_speed: 10.,
            run_speed: 30.,
            friction: 0.5,
            pitch: 0.,
            yaw: 0.,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in &mut query {
        if !options.enabled {
            continue;
        }

        let mut axis_input = Vec3::ZERO;

        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.;
        }

        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };

            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0., 1.);
            options.velocity *= 1. - friction;

            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }

        let forward = transform.forward();
        let right = transform.right();

        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        if mouse_delta != Vec2::ZERO {
            options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * options.sensitivity * dt;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    }
}

pub fn cursor_grab(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
) {
    let mut window = windows.get_single_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key_input.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
