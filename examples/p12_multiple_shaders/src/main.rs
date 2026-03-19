use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::{Material2d, Material2dPlugin},
    window::{PresentMode, WindowMode, WindowResolution},
};
const SHADER_A: &str = "a.wgsl";
const SHADER_B: &str = "b.wgsl";
const SHADER_C: &str = "c.wgsl";

fn main() {
    // Set the window properties
    let window_settings = Window {
        resolution: WindowResolution::new(800.0, 600.0),
        title: "P12 Multiple shaders".to_string(),
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
    app.add_plugins((
        Material2dPlugin::<ShaderAMat>::default(),
        Material2dPlugin::<ShaderBMat>::default(),
        Material2dPlugin::<ShaderCMat>::default(),
    ));
    app.add_systems(Startup, setup);
    app.add_systems(Update, time_system);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shader_a: ResMut<Assets<ShaderAMat>>,
    mut shader_b: ResMut<Assets<ShaderBMat>>,
    mut shader_c: ResMut<Assets<ShaderCMat>>,
) {
    // Create a 2D camera with orthographic projection
    let mut proj = OrthographicProjection::default_2d();
    proj.scale = 0.01;
    commands.spawn((Camera2d, proj));

    // Init shader materials
    let material_a = shader_a.add(ShaderAMat::default());
    let material_b = shader_b.add(ShaderBMat::default());
    let material_c = shader_c.add(ShaderCMat::default());

    // Create colored triangle mesh
    let mesh = create_colored_triangle();

    // Spawn the triangles
    commands.spawn((
        Mesh2d(meshes.add(mesh.clone())),
        MeshMaterial2d(material_a),
        Transform::from_translation(Vec3::new(-2.0, -0.5, 0.)),
    ));

    commands.spawn((
        Mesh2d(meshes.add(mesh.clone())),
        MeshMaterial2d(material_b),
        Transform::from_translation(Vec3::new(-0.5, -0.5, 0.)),
    ));

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(material_c),
        Transform::from_translation(Vec3::new(1., -0.5, 0.)),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
struct ShaderAMat {
    #[uniform(0)]
    time: f32,
}

impl Material2d for ShaderAMat {
    fn fragment_shader() -> ShaderRef {
        SHADER_A.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
struct ShaderBMat {
    #[uniform(0)]
    time: f32,
}

impl Material2d for ShaderBMat {
    fn fragment_shader() -> ShaderRef {
        SHADER_B.into()
    }
    fn vertex_shader() -> ShaderRef {
        SHADER_B.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
struct ShaderCMat {
    #[uniform(0)]
    time: f32,
}

impl Material2d for ShaderCMat {
    fn fragment_shader() -> ShaderRef {
        SHADER_C.into()
    }
    fn vertex_shader() -> ShaderRef {
        SHADER_C.into()
    }
}

fn create_colored_triangle() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        RenderAssetUsages::all(),
    );

    // Create triangle using Triangle2d
    let triangle = Triangle2d::new(Vec2::ZERO, Vec2::new(0.5, 1.), Vec2::new(1., 0.));

    // Convert triangle vertices to Vec<[f32; 3]>
    let positions: Vec<[f32; 3]> = triangle.vertices.iter().map(|v| [v.x, v.y, 0.0]).collect();
    // Define colors for each vertex
    let colors = vec![
        [1.0, 0.0, 0.0, 1.0], // Red
        [0.0, 1.0, 0.0, 1.0], // Green
        [0.0, 0.0, 1.0, 1.0], // Blue
    ];

    // Set vertex attributes
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    // Set indices for the triangle
    mesh.insert_indices(bevy::render::mesh::Indices::U32(vec![0, 1, 2]));

    mesh
}

fn time_system(
    time: Res<Time>,
    mut material_a: ResMut<Assets<ShaderAMat>>,
    mut material_b: ResMut<Assets<ShaderBMat>>,
    mut material_c: ResMut<Assets<ShaderCMat>>,
) {
    if let Some((_, material)) = material_a.iter_mut().next() {
        material.time = time.elapsed_secs();
    };
    if let Some((_, material)) = material_b.iter_mut().next() {
        material.time = time.elapsed_secs();
    };
    if let Some((_, material)) = material_c.iter_mut().next() {
        material.time = time.elapsed_secs();
    };
}
