#version 330 core

layout (location = 0) in int packedData; // Input packed data as an integer

out vec3 TexCoord; // The texture coordinates to pass to the fragment shader

uniform vec3 worldPosition;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    vec3 aPos = vec3(
        int((packedData >> 18) & 0x3F),
        int((packedData >> 12) & 0x3F),
        int((packedData >> 6) & 0x3F)
    );

    // Apply transformation and set the position
    gl_Position = projection * view * vec4(worldPosition + aPos, 1.0);

    int normal = (packedData >> 3) & 0x07;
    int textureID = packedData & 0x07;

    switch (normal) {
        case 0: // Front
            TexCoord = vec3(aPos.xy, float(textureID)); // Adjust UV for front
            break;
        case 1: // Back
            TexCoord = vec3(aPos.xy, float(textureID)); // Adjust UV for back
            break;
        case 2: // Left
            TexCoord = vec3(aPos.zy, float(textureID)); // Adjust UV for left
            break;
        case 3: // Right
            TexCoord = vec3(aPos.zy, float(textureID)); // Adjust UV for right
            break;
        case 4:
            TexCoord = vec3(aPos.xz, float(textureID)); // Adjust UV for top
            break;
        case 5:
            TexCoord = vec3(aPos.xz, float(textureID)); // Adjust UV for bottom
            break;
    }
}