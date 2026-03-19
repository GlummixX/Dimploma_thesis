#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

const line_frequency: f32 = 300.0;
const line_intensity: f32 = 0.2;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(screen_texture, texture_sampler, in.uv);

    let scanline: f32 = sin(in.uv.y * line_frequency * 3.14159) * line_intensity;

    color = vec4<f32>(color.rgb - color.rgb * scanline, 1.0);

    return color;
}
