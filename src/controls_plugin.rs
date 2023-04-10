use bevy::{app::AppExit, input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use std::f32::consts::FRAC_PI_2;

const MOUSE_MOTION_MULTIPLIER: f32 = 0.005;
const MOVE_SPEED: f32 = 5.0;
const SHIFTED_MOVE_SPEED_MULTIPLIER: f32 = 5.0;
const MIN_EULER_X: f32 = -FRAC_PI_2 + 0.005;
const MAX_EULER_X: f32 = FRAC_PI_2 - 0.005;

#[derive(Component)]
pub struct MainCamera {}

#[derive(Resource, Default)]
struct ApplyMouseMovements(bool);

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_controls)
            .add_system(close_on_escape)
            .add_system(cursor_grab_system)
            .insert_resource(ApplyMouseMovements(false));
    }
}

fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    mut apply_mouse_movements: ResMut<ApplyMouseMovements>,
) {
    let mut window = windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        apply_mouse_movements.0 = true;
    }

    if btn.just_released(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        apply_mouse_movements.0 = false;
    }
}

fn camera_controls(
    keys: Res<Input<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
    apply_mouse_movements: Res<ApplyMouseMovements>,
) {
    let transform_result = camera.get_single_mut();

    if let Ok(mut transform) = transform_result {
        for event in mouse_events.iter() {
            if apply_mouse_movements.0 {
                let dx = event.delta.x;
                let dy = event.delta.y;
                let (mut x, mut y, z) = transform.rotation.to_euler(EulerRot::default());
                x -= dx * MOUSE_MOTION_MULTIPLIER;
                y -= dy * MOUSE_MOTION_MULTIPLIER;

                y = y.max(MIN_EULER_X).min(MAX_EULER_X);

                println!("{}, {}, {}", x, y, z);

                let rotation = Quat::from_euler(EulerRot::default(), x, y, z);
                transform.rotation = rotation;
            }
        }
        let mut move_speed = MOVE_SPEED;
        if keys.pressed(KeyCode::LShift) {
            move_speed *= SHIFTED_MOVE_SPEED_MULTIPLIER;
        }
        if keys.pressed(KeyCode::W) {
            // move camera forward
            let forward = transform.forward();
            transform.translation += forward * time.delta_seconds() * move_speed;
        }
        if keys.pressed(KeyCode::A) {
            // move camera left
            let left = transform.left();
            transform.translation += left * time.delta_seconds() * move_speed;
        }
        if keys.pressed(KeyCode::S) {
            // move camera backward
            let forward = transform.forward();
            transform.translation += forward * time.delta_seconds() * -1. * move_speed;
        }
        if keys.pressed(KeyCode::D) {
            // move camera right
            let left = transform.left();
            transform.translation += left * time.delta_seconds() * -1. * move_speed;
        }
    }
}

fn close_on_escape(mut app_exit_events: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
