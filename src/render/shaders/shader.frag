#version 330 core

in vec3 TexCoord; // The texture coordinates from the vertex shader

out vec4 FragColor; // The final color output

uniform sampler2DArray textureArray; // Array of textures

void main()
{
    // Texture lookup using TexCoord
    FragColor = texture(textureArray, TexCoord);
}