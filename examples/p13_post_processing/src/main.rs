use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        settings::{Backends, RenderCreation, WgpuSettings},
        view::ViewTarget,
        MainWorld, RenderApp, RenderPlugin,
    },
    window::{PresentMode, WindowMode, WindowResolution},
};

use camera_plugin;

const SCANLINE_SHADER: &str = "scanline.wgsl";
const SEPIA_SHADER: &str = "sepia.wgsl";
const EDGE_SHADER: &str = "edge.wgsl";

fn main() {
    // Set the window properties
    let window_settings = Window {
        resolution: WindowResolution::new(800.0, 600.0),
        title: "P13 Post processing".to_string(),
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
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window_plugin).set(render_plugin));
    app.add_plugins((PostProcessPlugin, camera_plugin::CameraControlPlugin));
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        camera_plugin::ControlledCamera,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotates,
    ));

    // light
    commands.spawn(DirectionalLight {
        illuminance: 1000.,
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_secs());
        transform.rotate_z(0.15 * time.delta_secs());
    }
}

#[derive(Event)]
struct SwitchShaderEvent;

fn shader_switcher(
    mut events: EventWriter<SwitchShaderEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        events.send(SwitchShaderEvent);
    }
}

/// ==== Post process plugin ==== ///

struct PostProcessPlugin;

fn check_event(mut world: ResMut<MainWorld>, mut ppp: ResMut<PostProcessPipeline>) {
    let mut events = world
        .get_resource_mut::<Events<SwitchShaderEvent>>()
        .unwrap();
    let event = !events.is_empty();
    if event {
        events.clear();
        ppp.cycle_active();
    };
}

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchShaderEvent>();
        app.add_systems(Update, shader_switcher);

        // We need to get the render app from the main app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(ExtractSchedule, check_event);

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(Core3d, PostProcessLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    PostProcessLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

// Post process node for the render graph
#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode {
    type ViewQuery = &'static ViewTarget;

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline_def) = post_process_pipeline.get_active() else {
            return Ok(());
        };

        let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline_def.pipeline_id) else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &pipeline_def.layout,
            &BindGroupEntries::sequential((post_process.source, &pipeline_def.sampler)),
        );

        // Rrender pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // drawing a fullscreen triangle,
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

struct PipelineDef {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

#[derive(Resource)]
struct PostProcessPipeline {
    defs: Vec<PipelineDef>,
    active: usize,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let mut defs: Vec<PipelineDef> = Vec::new();

        defs.push(Self::add_pipeline(world, SCANLINE_SHADER.to_owned()));
        defs.push(Self::add_pipeline(world, SEPIA_SHADER.to_owned()));
        defs.push(Self::add_pipeline(world, EDGE_SHADER.to_owned()));

        Self { defs, active: 0 }
    }
}

impl PostProcessPipeline {
    fn get_active(&self) -> Option<&PipelineDef> {
        self.defs.get(self.active)
    }

    fn cycle_active(&mut self) {
        self.active += 1;
        if self.active >= self.defs.len() {
            self.active = 0;
        }
    }

    fn add_pipeline(world: &mut World, shader: String) -> PipelineDef {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let shader = world.load_asset(shader);

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("post_process_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                    zero_initialize_workgroup_memory: false,
                });

        PipelineDef {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
