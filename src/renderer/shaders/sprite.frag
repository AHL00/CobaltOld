in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D texture_;

void main()
{
    FragColor = texture(texture_, TexCoord);
    //FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}

