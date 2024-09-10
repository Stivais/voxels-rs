#version 330 core

in vec3 TexCoord;

out vec4 FragColor;

uniform sampler2DArray textureArray;

void main()
{
    // Texture lookup using TexCoord
    FragColor = texture(textureArray, TexCoord);
}