// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
#version 450

layout(location=0) in vec3 v_color;
layout(location=0) out vec4 f_color;

void main() {
    f_color = vec4(v_color * 0.5, 1.0);
}
