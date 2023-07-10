#version 330 core

in vec2 frag_uv;
in vec3 frag_color;

out vec4 out_color;

uniform sampler2D wood_tex;
uniform sampler2D marble_tex;
uniform sampler2D logo_tex;

void main() {
    vec4 marble = texture(marble_tex, frag_uv);
    vec4 wood = texture(wood_tex, frag_uv);
    vec4 logo = texture(logo_tex, frag_uv);

    out_color = wood * ((frag_uv.x + frag_uv.y) / 2) + marble * (((1 - frag_uv.x) + (1 - frag_uv.y)) / 2) + logo;
    out_color = out_color * 0.65 + vec4(frag_color, 1.0) * 0.35;
}