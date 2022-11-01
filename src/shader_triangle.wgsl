// shader in wgpu has to be specified in a binary format called SPIR-V
// we write in WGSL and convert them to SPIR-V binary

struct VOutput{   
    @location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VOutput {    
    var pos = array<vec2<f32>, 9>(
        vec2<f32>(-0.6,  0.0),
        vec2<f32>(-0.3,  0.0),
        vec2<f32>(-0.45,  0.3),
        vec2<f32>(-0.15,  0.0),
        vec2<f32>( 0.15,  0.0),
        vec2<f32>( 0.0,  0.3),
        vec2<f32>( 0.3,  0.0),
        vec2<f32>( 0.6,  0.0),
        vec2<f32>( 0.45,  0.3) 
    );
    var color = array<vec3<f32>, 9>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0)
    );

    var out: VOutput;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    out.v_color = vec4<f32>(color[in_vertex_index], 1.0);
    return out;
}

// color value is interpolated based on the position value it gets
@fragment
fn fs_main(in: VOutput) -> @location(0) vec4<f32> {
    return in.v_color;
}