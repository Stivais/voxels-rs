#version 330 core

in vec3 TexCoord;
in vec3 color;

out vec4 FragColor;

uniform sampler2DArray textureArray;

void main()
{
    // Texture lookup using TexCoord
//    FragColor = texture(textureArray, TexCoord);
    FragColor = vec4(color, 1);
}