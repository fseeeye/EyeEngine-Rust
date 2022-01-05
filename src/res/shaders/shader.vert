// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
#version 450

// from Vertex Buffer
layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
// from Instance Buffer, this will be different when shader process another instance
layout(location=5) in vec4 model_matrix_0;
layout(location=6) in vec4 model_matrix_1;
layout(location=7) in vec4 model_matrix_2;
layout(location=8) in vec4 model_matrix_3;

layout(location=0) out vec2 v_tex_coords;

layout(set=1, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

void main() {
    v_tex_coords = a_tex_coords;

    mat4 model_matrix = mat4(
        model_matrix_0,
        model_matrix_1,
        model_matrix_2,
        model_matrix_3
    );
    gl_Position = u_view_proj * model_matrix * vec4(a_position, 1.0);
}
