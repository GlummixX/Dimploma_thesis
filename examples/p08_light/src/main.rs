use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_obj::ObjPlugin;
use camera_plugin;

//https://sketchfab.com/3d-models/the-utah-teapot-1092c2832df14099807f66c8b792374d

const SHADER_FILE: &str = "shader.wgsl";

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
    app.add_plugins(MaterialPlugin::<PhongLight>::default());
    app.add_systems(Startup, setup);
    app.add_systems(Update, camera_update_system);
    app.add_systems(Update, esc_exit_system);
    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<PhongLight>>,
    asset_server: Res<AssetServer>,
) {
    // Create camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 20.),
        camera_plugin::ControlledCamera,
    ));

    // Create Phong
    let phong = PhongLight {
        light_pos: Vec3::new(5.0, 5.0, -5.0),
        camera_pos: Vec3::new(0.0, 0.0, 0.0),
        ambient_color: Vec3::splat(0.2),
        light_color: Vec3::new(1.0, 1.0, 1.0),
        specular_value: 32.0,
    };

    let mesh = Mesh3d(asset_server.load("teapot.obj"));
    commands.spawn((
        mesh,
        MeshMaterial3d(materials.add(phong)),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.05)),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PhongLight {
    #[uniform(0)]
    light_pos: Vec3,
    #[uniform(1)]
    camera_pos: Vec3,
    #[uniform(2)]
    ambient_color: Vec3,
    #[uniform(3)]
    light_color: Vec3,
    #[uniform(4)]
    specular_value: f32,
}

impl Material for PhongLight {
    fn fragment_shader() -> ShaderRef {
        SHADER_FILE.into()
    }
}

fn esc_exit_system(
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit::Success);
    }
}

fn camera_update_system(
    query: Query<&Transform, With<Camera3d>>,
    mut materials: ResMut<Assets<PhongLight>>,
) {
    if let Ok(transform) = query.get_single() {
        for (_, material) in materials.iter_mut() {
            material.camera_pos = transform.translation;
        }
    }
}
