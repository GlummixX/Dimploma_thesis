@group(0) @binding(0)
var<storage, read_write> vertices: array<vec2<f32>>;

@group(0) @binding(1)
var<storage, read_write> velocities: array<vec2<f32>>;

@group(0) @binding(2)
var<uniform> uniform_data: vec4<f32>;

const maxSpeed: f32 = 10.0;
const minLength: f32 = 0.1;
const friction: f32 = -1;

@compute @workgroup_size(256, 1, 1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let index: u32 = GlobalInvocationID.x;

    var vel: vec2<f32> = velocities[index];
    var pos: vec2<f32> = vertices[index] + uniform_data.z * vel;

    if (abs(pos.x) > 1.0) {
        vel.x = sign(pos.x) * (-0.95 * abs(vel.x) - 0.0001);
        pos.x = clamp(pos.x, -1.0, 1.0);
    }

    if (abs(pos.y) > 1.0) {
        vel.y = sign(pos.y) * (-0.95 * abs(vel.y) - 0.0001);
        pos.y = clamp(pos.y, -1.0, 1.0);
    }

    let t: vec2<f32> = uniform_data.xy - pos;
    let r: f32 = max(length(t), minLength);
    let force: vec2<f32> = uniform_data.w * (t / r) / (r * r);

    vel = vel + uniform_data.z * force;

    if (length(vel) > maxSpeed) {
        vel = maxSpeed * normalize(vel);
    }

    vertices[index] = pos;
    velocities[index] = vel * exp(friction * uniform_data.z);
}
