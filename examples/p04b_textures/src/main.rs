use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef}, settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin
    },
};

const SHADER_PATH: &str = "shader.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TexturedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material for TexturedMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

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
    app.add_plugins(MaterialPlugin::<TexturedMaterial>::default());
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate_system);
    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TexturedMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0., 1., 20.)));
    
    // Load texture
    let texture_handle = asset_server.load("earth.png");

    // Setup the material 
    let material = materials.add(TexturedMaterial{texture:texture_handle});

    // Spawn Sphere
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
