// doc: https://www.w3.org/TR/WGSL/

/// Vertex shader

// the output of vertex shader
struct VertexOutput {
    // The [[builtin(position)]] bit tells WGPU that this value we want to use as the vertex's clip coordinates,
    // which is analogous to GLSL's `gl_Position` variable.
    [[builtin(position)]] clip_position: vec4<f32>;
};

// `[[stage(vertex)]]` mark this function as a valid entry point for a vertex shader.
[[stage(vertex)]]
fn vs_main(
    // expect a u32 called in_vertex_index which gets its value from [[builtin(vertex_index)]].
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    // declare output struct
    // tips: Variables defined with `var` can be modified, but must specify their type.
    var out: VertexOutput;

    // x & y of a triangle
    // tips: Variables created with `let` can have their types inferred, but their value cannot be changed during the shader.
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    return out;
}


/// Fragment shader

// newer versions of the WGSL spec require these entry point names to be different.
// we will spec the entry point when we create Render Pipeline in Application::new()
// WGSL spec ref: https://www.w3.org/TR/WGSL/#declaration-and-scope
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // sets the color of the current fragment
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}