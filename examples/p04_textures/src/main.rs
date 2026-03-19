use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};

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
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate_system);
    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0., 1., 20.)));
    // Create light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 1., 10.0),
    ));

    // Load texture
    let texture_handle = asset_server.load("earth.png");

    // Assign the texture to the material
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        reflectance: 0.3,
        perceptual_roughness: 0.8,
        ..Default::default()
    });

    // Sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(5.))),
        MeshMaterial3d(material),
        Transform::from_xyz(0., 0., 0.),
        Rotate{}
    ));
}

#[derive(Component)]
struct Rotate {}

fn rotate_system(mut query: Query<&mut Transform, With<Rotate>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_y(0.01);
    }
}
