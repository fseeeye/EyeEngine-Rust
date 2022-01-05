// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

layout(set=1, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

void main() {
    v_tex_coords = a_tex_coords;
    // gl_VertexIndex: the index of the current vertex in the vertex data.
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}
