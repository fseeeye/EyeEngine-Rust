// doc: https://www.w3.org/TR/WGSL/

/// Vertex shader

// the input of vertex shader
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

// the output of vertex shader
struct VertexOutput {
    // The [[builtin(position)]] bit tells WGPU that this value we want to use as the vertex's clip coordinates,
    // which is analogous to GLSL's `gl_Position` variable.
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

// `[[stage(vertex)]]` mark this function as a valid entry point for a vertex shader.
[[stage(vertex)]]
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    // declare output struct
    // tips: Variables defined with `var` can be modified, but must specify their type.
    var out: VertexOutput;

    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);

    return out;
}


/// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

// newer versions of the WGSL spec require these entry point names to be different.
// we will spec the entry point when we create Render Pipeline in Application::new()
// WGSL spec ref: https://www.w3.org/TR/WGSL/#declaration-and-scope
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // sets the color of the current fragment
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}