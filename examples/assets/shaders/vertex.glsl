#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec2 uv;

out vec2 frag_uv;
out vec3 frag_color;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    frag_color = color;
    frag_uv = uv;
}