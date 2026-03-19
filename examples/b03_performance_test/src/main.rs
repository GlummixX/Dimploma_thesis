use std::time::Instant;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::{
        pipelined_rendering::PipelinedRenderingPlugin, settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin
    }, window::{PresentMode, WindowMode, WindowResolution}
};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Component, Default)]
struct RotateComponent {
    speed: f32,
}

#[derive(Resource)]
struct BenchmarkData {
    start: Instant,
    data: Vec<f32>,
}
impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            data: Default::default(),
        }
    }
}
impl BenchmarkData {
    fn add_entry(&mut self, frametime: f32) -> bool {
        if self.data.len() == 0 {
            self.start = Instant::now();
        }
        self.data.push(frametime);
        Instant::now().duration_since(self.start).as_secs_f32() > 30.0
    }
    fn get_data(&mut self) -> Vec<f32> {
        self.data.remove(0);
        self.data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let minimum = self.data.get(0).unwrap();
        let maximum = self.data.get(self.data.len()-1).unwrap();
        let median = self.data.get((self.data.len() / 2)-1).unwrap();
        let pct95 = self
            .data
            .get((self.data.len() as f32 * 0.95) as usize)
            .unwrap();
        let pct5 = self
            .data
            .get((self.data.len() as f32 * 0.05) as usize)
            .unwrap();
        let avg = self.data.iter().sum::<f32>() / self.data.len() as f32;
        vec![*maximum, *minimum, *median, *pct5, *pct95, avg]
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let be;
    match args[1].as_str() {
        "vk" => be = Backends::VULKAN,
        "dx12" => be = Backends::DX12,
        "gl" => be = Backends::GL,
        "metal" => be = Backends::METAL,
        "wgpu" => be = Backends::BROWSER_WEBGPU,
        _ => panic!("Unsupported BE"),
    };
    let window_settings = Window {
        resolution: WindowResolution::new(800.0, 600.0),
        title: "B03 Performance test".to_string(),
        mode: WindowMode::Windowed,
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    };
    let window_plugin = WindowPlugin {
        primary_window: Some(window_settings),
        ..default()
    };
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(be),
            power_preference: bevy::render::settings::PowerPreference::HighPerformance,
            ..default()
        }),
        ..default()
    };
    let mut default_plugins = DefaultPlugins.set(render_plugin).set(window_plugin);
    if let Some(x) = args.get(2) {
        if x.as_str() == "np"{
            default_plugins = default_plugins.build().disable::<PipelinedRenderingPlugin>();
        }
        
    }
    let mut app = App::new();
    app.add_plugins(default_plugins);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.insert_resource(BenchmarkData::default());
    app.add_systems(Update, perf_system);
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate_shapes);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera creation
    commands.spawn((Camera3d::default(), Transform::from_xyz(0., 0., 25.)));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 1., 10.0),
    ));
    let mut rng = StdRng::seed_from_u64(42);
    let shapes = vec![
        meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        meshes.add(Sphere::new(0.25)),
        meshes.add(Torus::new(0.125, 0.25)),
    ];

    let mut colors = vec![];
    for _ in 0..32{
        colors.push(
            materials.add(StandardMaterial {
                base_color: Color::linear_rgb(
                    rng.gen_range(0.0..=1.0),
                    rng.gen_range(0.0..=1.0),
                    rng.gen_range(0.0..=1.0),
                ),
                perceptual_roughness: 0.5,
                ..default()
            })
        );
    }

    for _ in 0..5000 {
        let shape_index = rng.gen_range(0..shapes.len());
        let color_index = rng.gen_range(0..colors.len());
        let position = Vec3::new(
            rng.gen_range(-7.5..7.5),
            rng.gen_range(-7.5..7.5),
            rng.gen_range(-7.5..7.5),
        );

        let mesh_handle = shapes[shape_index].clone();
        let material_handle = colors[color_index].clone();

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(material_handle),
            Transform::from_translation(position),
            GlobalTransform::default(),
            RotateComponent {
                speed: rng.gen_range(-10.0..10.0),
            },
        ));
    }
}

// Basic entity rotation system like in b02
fn rotate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &RotateComponent)>) {
    for (mut transform, rotate) in &mut query {
        transform.rotate_z(rotate.speed * time.delta_secs());
    }
}

fn perf_system(
    mut res: ResMut<BenchmarkData>,
    time: Res<Time>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if res.add_entry(time.delta_secs()) {
        print!("{:?}", res.get_data());
        exit_events.send(bevy::app::AppExit::Success);
    }
}
