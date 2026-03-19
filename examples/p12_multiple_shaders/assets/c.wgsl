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
    var angle: f32 = time + in.position.x;
    var radius: f32 = 1.5;
    var circularMotion: vec2<f32> = vec2<f32>(cos(angle), sin(angle)) * radius;
    var transformedPosition: vec3<f32> = vec3<f32>(in.position.xy + circularMotion, -1.0);

    var out: VertexOutput;
    out.position = mesh2d_position_local_to_clip(get_world_from_local(in.instance_index), vec4<f32>(transformedPosition, 1.0));
    out.color = in.color;
    return out;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = in.color;
    color.r = color.r * (sin(time * 2.0) * 0.5 + 0.5);
    color.g = color.g * (cos(time * 2.0) * 0.5 + 0.5);
    color.b = color.b * (sin(time * 3.0) * 0.5 + 0.5);
    return color;
}