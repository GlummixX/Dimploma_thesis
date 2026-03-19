#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip};

@group(2) @binding(0) var<uniform> time: f32;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(4) color: vec4<f32>,
};

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(4) color: vec4<f32>,
};

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    var scale_factor: f32 = sin(time) * 0.5 + 1.0;
    var scaled_pos: vec3<f32> = vec3<f32>(in.position.xy, -1.0) * scale_factor;
    scaled_pos.y += sin(time + in.position.x) * 0.2;

    var out: VertexOutput;
    out.position = mesh2d_position_local_to_clip(get_world_from_local(in.instance_index), vec4<f32>(scaled_pos, 1.0));
    out.color = in.color * (0.5 + 0.5 * sin(time));
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}