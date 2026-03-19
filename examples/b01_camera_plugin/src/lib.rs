use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Component)]
pub struct ControlledCamera;

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller_system);
    }
}

fn camera_controller_system(
    mut query: Query<&mut Transform, (With<Camera3d>, With<ControlledCamera>)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = primary_window.get_single() {
        if let Ok(mut transform) = query.get_single_mut() {
            let mut input = Vec3::ZERO;
            if keyboard.pressed(KeyCode::KeyW) {
                input.z -= time.delta_secs();
            }
            if keyboard.pressed(KeyCode::KeyS) {
                input.z += time.delta_secs();
            }
            if keyboard.pressed(KeyCode::KeyA) {
                input.x -= time.delta_secs();
            }
            if keyboard.pressed(KeyCode::KeyD) {
                input.x += time.delta_secs();
            }
            if keyboard.pressed(KeyCode::Space) {
                input.y += time.delta_secs();
            }
            if keyboard.pressed(KeyCode::KeyC) {
                input.y -= time.delta_secs();
            }
            if input != Vec3::ZERO {
                let by = transform.rotation * input * 5.;
                transform.translation += by;
            }
            for ev in mouse.read() {
                let (mut azim, mut zeni, _) = transform.rotation.to_euler(EulerRot::YXZ);
                let win_size = window.width().min(window.height());
                zeni -= (100. * ev.delta.y / win_size).to_radians();
                azim -= (100. * ev.delta.x / win_size).to_radians();
                zeni = zeni.clamp(-1.54, 1.54);

                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, azim) * Quat::from_axis_angle(Vec3::X, zeni)
            }      
        }
    }
}