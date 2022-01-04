// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
#version 450

layout(location=0) in vec2 v_tex_coords;
// - out: the value is meant to be written to a buffer to be used outside the shader program.
// - layout: specify a layout for the variable.
// In this case, the value of `f_color` will be saved to whatever buffer is at location zero in our application
// In most cases, location=0 is the current texture from the swapchain aka the screen.
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
}
