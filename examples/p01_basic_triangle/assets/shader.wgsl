
struct FragmentInput {
    @location(4) frag_color: vec4<f32>
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return  in.frag_color;
}