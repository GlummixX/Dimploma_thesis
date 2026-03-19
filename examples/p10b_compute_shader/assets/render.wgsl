@group(0) @binding(0)
var<storage> vertices: array<vec2<f32>>;

@group(0) @binding(1)
var<storage> velocities: array<vec2<f32>>;

const maxSpeed: f32 = 10.0;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
    // Debug: Always output red color and center position
    var output: VertexOutput;
    var pos: vec2<f32> = vertices[vertexIndex];
    output.position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);  // Center of screen
    output.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);     // Red
    return output;
}

@fragment
fn fs_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
