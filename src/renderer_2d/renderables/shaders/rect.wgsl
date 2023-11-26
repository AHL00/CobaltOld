
struct CameraUniform {
    @location(0) view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> model: mat4x4<f32>;

@group(2) @binding(0)
var<uniform> color: vec4<f32>;

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

    out.clip_position = camera.view_proj * model * vec4<f32>(in.position, 1.0);
    out.uv = in.uv;

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Either sprites need to be sorted, or alpha testing needs to be implemented
    // Just depth buffer doesn't work because on sprites with transparent pixels,
    // those still have depth values.
    // So when another sprite is rendered behind these transparent pixels,
    // depth testing fails because the transparent pixels are closer to the camera.

    // Is 0.01 a good value?
    // Maybe it should be a parameter of the shader?
    // TODO: Add alpha testing uniform
    if (color.a < 0.01) {
        discard;
        // Discarding this fragment means the depth value for this pixel will
        // not be updated, so the transparent pixels will not block other sprites
    }

    return color;
}