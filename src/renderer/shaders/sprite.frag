#version 460 core

in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D sprite_texture;

void main()
{
    FragColor = texture(sprite_texture, TexCoord);
}

