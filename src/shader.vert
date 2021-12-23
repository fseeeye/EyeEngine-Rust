// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
#version 450

// currently, we store vertex data in the shader as positions
const vec2 positions[3] = vec2[3](
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
);

void main() {
    // gl_VertexIndex: the index of the current vertex in the vertex data.
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
}
