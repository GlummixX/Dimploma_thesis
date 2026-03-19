@group(0) @binding(0)
var<storage, read> inputData: array<i32>;

@group(0) @binding(1)
var<storage, read_write> result: array<i32>;

@compute @workgroup_size(256, 1, 1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let index = GlobalInvocationID.x;
    let count = arrayLength(&inputData); // May not be available everywhere

    // Fallback strategy if arrayLength isn't available:
    // let count: u32 = ... ; // pass this as a uniform

    var sum: i32 = 0;
    var min_val: i32 = inputData[0];
    var max_val: i32 = inputData[0];

    for (var i: u32 = 0u; i < count; i = i + 1u) {
        let val = i32(inputData[i]);
        sum = sum + val;
        if (val < min_val) {
            min_val = val;
        }
        if (val > max_val) {
            max_val = val;
        }
    }

    result[0] = sum;
    result[1] = min_val;
    result[2] = max_val;
}
