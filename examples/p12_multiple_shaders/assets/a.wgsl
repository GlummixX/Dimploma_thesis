@group(2) @binding(0) var<uniform> time: f32;

struct FragmentInput {
    @location(4) color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var r:f32 = sin(time) * in.color.r + 0.5;
    var g:f32 = sin(time + 2.0) * in.color.g + 0.5;
    var b:f32 = sin(time + 4.0) * in.color.b + 0.5;

    return vec4<f32>(r, g, b, 1.0);
}