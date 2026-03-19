use bevy::{
    prelude::*,
    render::{
        camera::ClearColor,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        gpu_readback::{Readback, ReadbackComplete},
        render_asset::RenderAssets,
        render_graph::{GraphInput, Node, RenderGraph, RenderLabel},
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroup, BindGroupLayout, BindGroupLayoutEntry,
            BindingType, BufferUsages, CachedComputePipelineId, ComputePassDescriptor,
            ComputePipelineDescriptor, PipelineCache, ShaderStages,
        },
        renderer::RenderDevice,
        settings::{Backends, RenderCreation, WgpuSettings},
        storage::{GpuShaderStorageBuffer, ShaderStorageBuffer},
        texture::{FallbackImage, GpuImage},
        Render, RenderApp, RenderPlugin, RenderSet,
    },
    DefaultPlugins,
};

use rand::prelude::*;
use std::borrow::Cow;

const SHADER_ASSET_PATH: &str = "shader.wgsl";

fn main() {
    let render_plugin = RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..default()
        }),
        ..default()
    };
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK));
    app.add_plugins((DefaultPlugins.set(render_plugin), ComputePlugin));
    app.add_plugins(ExtractResourcePlugin::<ComputeBuffers>::default());
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands, mut buff: ResMut<Assets<ShaderStorageBuffer>>) {
    let num = 100;
    let mut rng = rand::rng();
    let mut data: Vec<i32> = Vec::with_capacity(num);
    for _ in 0..data.capacity() {
        data.push(rng.random_range(-10..10));
    }
    let mut output = ShaderStorageBuffer::from(vec![0, 0, 0]);
    output.buffer_description.usage |= BufferUsages::COPY_SRC;
    let output_handle = buff.add(output);

    println!("==============================");
    println!(
        "Expected: sum:{} min:{} max:{}",
        data.iter().sum::<i32>(),
        data.iter().min().unwrap(),
        data.iter().max().unwrap()
    );

    commands
        .spawn(Readback::buffer(output_handle.clone()))
        .observe(
            |trigger: Trigger<ReadbackComplete>,
             mut exit_events: ResMut<Events<bevy::app::AppExit>>| {
                let contents: Vec<i32> = trigger.event().to_shader_type();
                println!("Readback result: {:?}", contents);
                if contents[0] != contents[1] && contents[1] != contents[2] {
                    println!(
                        "Result: sum:{} min:{} max:{}",
                        contents[0], contents[1], contents[2]
                    );
                    exit_events.send(AppExit::Success);
                }
            },
        );

    commands.insert_resource(ComputeBuffers {
        input: buff.add(data),
        output: output_handle,
    });
}

#[derive(RenderLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ComputeNode;

pub struct ComputePlugin; //Custom compute plugin

impl Plugin for ComputePlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        // This function is called when the app is finished initializing this plugin.
        if let Some(app) = app.get_sub_app_mut(RenderApp) {
            app.init_resource::<ComputePipeline>(); //Custom compute pipeline
            app.add_systems(Render, prepare_buffers.in_set(RenderSet::Prepare)); //Prepare buffers in the prep stage of Render pass

            let mut render_graph = app.world_mut().get_resource_mut::<RenderGraph>().unwrap();

            //Add and connect custom compute node to the render graph
            render_graph.add_node(ComputeNode, DispatchCompute {});
            let r = render_graph.try_add_node_edge(GraphInput, ComputeNode);
            if r.is_err() {
                println!("{:?}", r);
            }
        }
    }
}

#[derive(Resource, AsBindGroup, Debug, Clone, ExtractResource)]
struct ComputeBuffers {
    #[storage(0, visibility(compute))]
    input: Handle<ShaderStorageBuffer>,
    #[storage(1, visibility(compute))]
    output: Handle<ShaderStorageBuffer>,
}

#[derive(Resource)]
struct ComputeBindGroup(pub BindGroup);

fn prepare_buffers(
    mut commands: Commands,
    pipeline: Res<ComputePipeline>,
    buffers: Res<ComputeBuffers>,
    render_device: Res<RenderDevice>,
    render_assets_images: Res<RenderAssets<GpuImage>>,
    fallback_image: Res<FallbackImage>,
    render_assets_buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
) {
    let mut param = (render_assets_images, fallback_image, render_assets_buffers);
    let result = buffers.as_bind_group(&pipeline.bind_group_layout, &render_device, &mut param);
    match result {
        Ok(buffers) => {
            commands.insert_resource(ComputeBindGroup(buffers.bind_group));
        }
        Err(AsBindGroupError::RetryNextUpdate) => {
            println!("retry next update");
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

#[derive(Resource)]
struct ComputePipeline {
    bind_group_layout: BindGroupLayout,
    compute_pipeline: CachedComputePipelineId,
}

impl FromWorld for ComputePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let bind_group_layout = render_device.create_bind_group_layout(
            "ComputeBuffers",
            &vec![
                // Input buffer (read-only storage buffer)
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Storage {
                            read_only: true,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Output buffer (write-only storage buffer)
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Storage {
                            read_only: false,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);

        let pipeline_cache = world.resource::<PipelineCache>();
        let compute_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("main"),
            zero_initialize_workgroup_memory: false,
        });

        ComputePipeline {
            bind_group_layout,
            compute_pipeline,
        }
    }
}

//Render graph node, that handles the compute pass
pub struct DispatchCompute;

impl Node for DispatchCompute {
    fn run(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let kernel_pipeline = world.get_resource::<ComputePipeline>();
        let kernel_bind_group = world.get_resource::<ComputeBindGroup>();
        let pipeline_cache = world.get_resource::<PipelineCache>();
        if let (Some(kernel_pipeline), Some(kernel_bind_group), Some(pipeline_cache)) =
            (kernel_pipeline, kernel_bind_group, pipeline_cache)
        {
            let mut pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("Compute Pass"),
                        ..Default::default()
                    });
            if let Some(real_pipeline) =
                pipeline_cache.get_compute_pipeline(kernel_pipeline.compute_pipeline)
            {
                pass.set_pipeline(&real_pipeline);
                pass.set_bind_group(0, &kernel_bind_group.0, &[]);
                pass.dispatch_workgroups(256, 1, 1);
            }
        }
        Ok(())
    }
}
