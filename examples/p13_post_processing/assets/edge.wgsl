#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

// Threshold for edge detection
const edge_threshold: f32 = 0.015;

const sobel_x: mat3x3<f32> = mat3x3<f32>(
    vec3<f32>(-1.0, -2.0, -1.0),
    vec3<f32>( 0.0,  0.0,  0.0),
    vec3<f32>( 1.0,  2.0,  1.0)
);

const sobel_y: mat3x3<f32> = mat3x3<f32>(
    vec3<f32>(-1.0,  0.0,  1.0),
    vec3<f32>(-2.0,  0.0,  2.0),
    vec3<f32>(-1.0,  0.0,  1.0)
);

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {

    var color_x: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var color_y: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let offset: vec2<f32> = vec2<f32>(1.0 / 1024.0);

    for (var i: i32 = 0; i < 3; i = i + 1) {
        for (var j: i32 = 0; j < 3; j = j + 1) {
            let sample_pos: vec2<f32> = in.uv + vec2<f32>(f32(i - 1), f32(j - 1)) * offset;
            let tex_color: vec3<f32> = textureSample(screen_texture, texture_sampler, sample_pos).rgb;
            let grayscale: f32 = dot(tex_color, vec3<f32>(0.299, 0.587, 0.114));

            color_x = color_x + grayscale * sobel_x[j][i];
            color_y = color_y + grayscale * sobel_y[j][i];
        }
    }

    let edge_intensity: f32 = length(color_x + color_y);
    let edge: f32 = select(0.0, 1.0, edge_intensity > edge_threshold);
    return vec4<f32>(1.0 - vec3<f32>(edge), 1.0);
}