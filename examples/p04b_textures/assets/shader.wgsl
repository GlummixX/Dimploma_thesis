#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0)
var color_texture: texture_2d<f32>;
@group(2) @binding(1)
var color_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_sampler, in.uv);
}
