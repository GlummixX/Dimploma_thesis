#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color: vec4<f32> = textureSample(screen_texture, texture_sampler, in.uv);

    var gray: f32 = dot(color.rgb, vec3<f32>(0.3, 0.59, 0.11));
    var sepia: vec3<f32> = vec3<f32>(gray) * vec3<f32>(1.2, 1.0, 0.8);

    sepia.r = clamp(sepia.r, 0.0, 1.0);
    sepia.g = clamp(sepia.g, 0.0, 1.0);
    sepia.b = clamp(sepia.b, 0.0, 1.0);
    return vec4<f32>(sepia, color.a);
}