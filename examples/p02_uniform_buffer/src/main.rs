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
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts, EguiPlugin,
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
            backends: Some(Backends::GL),
            ..default()
        }),
        ..default()
    };
    // Create the app
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window_plugin).set(render_plugin));
    app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    app.add_plugins(EguiPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, ui_example_system);
    app.run();
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
    let material = CustomMaterial {
        color_mul: LinearRgba::new(0.0, 1.0, 1.0, 1.0),
    };
    let handle = materials.add(material);
    let reference = MaterialReference(handle.clone());

    // Spawn the triangle
    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(handle),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));
    commands.insert_resource(reference);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color_mul: LinearRgba,
}
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_FILE.into()
    }
}
#[derive(Resource)]
struct MaterialReference(Handle<CustomMaterial>);

fn ui_example_system(
    mut contexts: EguiContexts,
    mat_ref: Res<MaterialReference>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    if let Some(material) = materials.get_mut(&mat_ref.0) {
        let mut red = material.color_mul.red;
        let mut green = material.color_mul.green;
        let mut blue = material.color_mul.blue;
        egui::Window::new("Vertex color multiplier").show(contexts.ctx_mut(), |ui| {
            egui::widgets::Slider::new(&mut red, 0.0..=1.0)
                .show_value(true)
                .text("Red")
                .ui(ui);
            egui::widgets::Slider::new(&mut green, 0.0..=1.0)
                .show_value(true)
                .text("Green")
                .ui(ui);
            egui::widgets::Slider::new(&mut blue, 0.0..=1.0)
                .show_value(true)
                .text("Blue")
                .ui(ui);
        });
        material.color_mul = LinearRgba::new(red, green, blue, 1.0);
    }
}
