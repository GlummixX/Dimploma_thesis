use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, PrimitiveTopology},
        render_resource::{AsBindGroup, ShaderRef, VertexFormat},
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::{Material2d, Material2dPlugin},
    window::{PresentMode, WindowMode, WindowResolution},
};

const SHADER_FILE: &str = "shader.wgsl";

fn main() {
    // Set the window properties
    let window_settings = Window {
        resolution: WindowResolution::new(800.0, 600.0),
        title: "P01 Basic triangle".to_string(),
        mode: WindowMode::Windowed,
        present_mode: PresentMode::AutoVsync,
        ..default()
    };
    let window_plugin = WindowPlugin {
        primary_window: Some(window_settings),
        ..default()
    };
    // Set the rendering backend
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        }),
        ..default()
    };
    // Create the app
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window_plugin).set(render_plugin));
    app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    app.add_systems(Startup, setup);
    app.run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_FILE.into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Create a 2D camera
    commands.spawn(Camera2d::default());

    // Create a triangle mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

    // Define the vertices for the triangle
    mesh.insert_attribute(
        MeshVertexAttribute::new("Vertex_Position", 0, VertexFormat::Float32x2),
        vec![[0.0, 100.0], [100.0, 0.0], [-100.0, 0.0]],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ],
    );

    // Init shader material
    let material = CustomMaterial {};

    // Spawn the triangle
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(material)),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));
}
