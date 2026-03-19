use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    }
};
use bevy_obj::ObjPlugin;
use camera_plugin;

//https://sketchfab.com/3d-models/the-utah-teapot-1092c2832df14099807f66c8b792374d

fn main() {
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        }),
        ..default()
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(render_plugin));
    app.add_plugins((ObjPlugin, camera_plugin::CameraControlPlugin));
    app.add_systems(Startup, setup);
    app.add_systems(Update, esc_exit_system);
    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 20.),
        camera_plugin::ControlledCamera,
    ));
    // Create light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 1500.0,
            ..Default::default()
        },
        Transform::from_xyz(25.0, 30.0, 35.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = Mesh3d(asset_server.load("teapot.obj"));
    commands.spawn((
        mesh,
        MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.05)),
    ));
}

fn esc_exit_system(
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit::Success);
    }
}
