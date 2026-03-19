// Phong shader

@group(2) @binding(0) var<uniform> light_pos: vec3<f32>;
@group(2) @binding(1) var<uniform> camera_pos: vec3<f32>;
@group(2) @binding(2) var<uniform> ambient_color: vec3<f32>;
@group(2) @binding(3) var<uniform> light_color: vec3<f32>;
@group(2) @binding(4) var<uniform> specular_value: f32;

struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let normal = normalize(in.normal);
    let light_dir = normalize(light_pos - in.world_position);
    let view_dir = normalize(camera_pos - in.world_position);
    let reflect_dir = reflect(-light_dir, normal);

    // Phong
    let ambient = ambient_color;
    let diffuse = max(dot(normal, light_dir), 0.0) * light_color;
    let specular = pow(max(dot(view_dir, reflect_dir), 0.0), specular_value) * light_color;

    let final_color = ambient + diffuse + specular;
    return vec4<f32>(final_color, 1.0);
}
