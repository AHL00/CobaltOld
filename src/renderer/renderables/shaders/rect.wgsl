// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(in.position, 1.0);
    out.uv = in.uv;

    return out;
}

// Fragment shader

@group(0) @binding(0)
var texture: texture_2d<f32>;

@group(0) @binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, in.uv);
}