#version 430 core
#extension GL_ARB_gpu_shader_int64 : enable


layout(binding = 0, std430) readonly buffer ssbo1 {
    uint64_t data[];
};

out vec3 TexCoord;
out vec3 color;

uniform vec3 worldPosition;
uniform mat4 view;
uniform mat4 projection;

const int flipLookup[6] = int[6](1, -1, 1, -1, 1, -1);

const int width_multipler[3] = int[3](38, 34, 52);
const int height_multplier[2] = int[2](28, 13);

const ivec2 test[2] = ivec2[2](
    ivec2(22, 11),
    ivec2(26, 52)
);

const vec2 facePositions[4] = vec2[4]
(
    vec2(0, 1),
    vec2(1, 0),
    vec2(1, 1),
    vec2(0, 0)
);

// Winding order to access the face positions
int indices[6] = int[6](0, 2, 1, 1, 3, 0);

void main()
{
    int index = gl_VertexID / 6;
    uint64_t packedData = data[index];
    int currVertexID = gl_VertexID % 6;

    uint x = uint((packedData) & 0x3F);
    uint y = uint((packedData >> 6) & 0x3F);
    uint z = uint((packedData >> 12) & 0x3F);

    int width = int((packedData >> 18) & 0x3F);
    int height = int((packedData >> 24) & 0x3F);

    uint normal = uint((packedData >> 30) & 0x07);
    uint textureID = uint((packedData >> 33) & 0x0F);

    // fastest way I could think of
    int w_multi = (test[currVertexID % 2].x >> currVertexID) & 1;
    int h_multi = (test[0].y >> currVertexID) & 1;

    vec3 position = vec3(x, y, z);

    switch (normal) {
        case 0: // Front
            position.xy += (facePositions[indices[currVertexID]] * vec2(width, height));
            TexCoord = vec3(position.xy, float(textureID));
            color = vec3(1.0, 0.0, 0.0); // red
            break;
        case 1: // Back
            position.xy += (facePositions[indices[currVertexID]] * vec2(width * -1, height));
            TexCoord = vec3(position.xy, float(textureID));
            color = vec3(0.0, 1.0, 0.0); // green
            break;
        case 2: // Left
            position.zy += (facePositions[indices[currVertexID]] * vec2(width * -1, height));
            TexCoord = vec3(position.zy, float(textureID));
            color = vec3(0.0, 0.0, 1.0); // blue
            break;
        case 3: // Right
            position.zy += (facePositions[indices[currVertexID]] * vec2(width, height));
            TexCoord = vec3(position.zy, float(textureID));
            color = vec3(1.0, 1.0, 0.0); // yellow
            break;
        case 4: // Top
            position.xz += (facePositions[indices[currVertexID]] * vec2(width, height));
            TexCoord = vec3(position.xz, float(textureID));
            color = vec3(1.0, 0.0, 1.0); // magenta
            break;
        case 5: // Bottom
            position.xz += (facePositions[indices[currVertexID]] * vec2(width * -1, height));
            TexCoord = vec3(position.xz, float(textureID));
            color = vec3(0.0, 0.0, 0.0); // black
            break;
    }

    gl_Position = projection * view * vec4(worldPosition + position, 1.0);
}
