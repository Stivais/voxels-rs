#version 460 core
#extension GL_ARB_gpu_shader_int64 : enable

layout(binding = 0, std430) readonly buffer ssbo1 {
    uint64_t data[];
};

out vec3 TexCoord;
out vec3 color;

//uniform mat4 view;
//uniform mat4 projection;
uniform mat4 view_projection;

const vec3 colorLookup[6] = {
    vec3(1.0, 0.0, 1.0),
    vec3(0.0, 0.0, 0.0),
    vec3(1.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0),
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0)
};

const int flipLookup[6] = int[6](1, -1, 1, 1, -1, 1);

void main()
{
    ivec3 chunkOffset = (ivec3(gl_BaseInstance&255u, gl_BaseInstance>>10&255u, gl_BaseInstance>>20&255u)) * 32;

    int vertexID = gl_VertexID % 4;
    int index = gl_VertexID >> 2u;

    uint64_t packedData = data[index];

    uint face = uint((packedData >> 30) & 0x07);

    ivec3 vertexPos = ivec3(packedData, packedData >> 6u, packedData >> 12u) & 63;

    int w = int((packedData >> 18u) & 63u), h = int((packedData >> 24u) & 63u);
    uint wDir = (face & 2) >> 1, hDir = 2 - (face >> 2);
    int wMod = vertexID >> 1, hMod = vertexID & 1;

    vertexPos[wDir] += (w * wMod * flipLookup[face]);
    vertexPos[hDir] += (h * hMod);

    vec3 position = vec3((vertexPos += chunkOffset));
    position[wDir] += 0.0007 * flipLookup[face] * (wMod * 2 - 1);
    position[hDir] += 0.0007 * (hMod * 2 - 1);

    uint textureID = uint((packedData >> 33) & 0x0F);

    color = colorLookup[face];
    // todo: figure out a way to get texCoord with switch statement
//    switch (face) {
//        case 0: // Top / 0 b4 4
//            TexCoord = vec3(position.xz, float(textureID));
//            color = vec3(1.0, 0.0, 1.0); // magenta
//            break;
//        case 1: // Bottom (Black) 1 b4 5
//            TexCoord = vec3(position.xz, float(textureID));
//            color = vec3(0.0, 0.0, 0.0);
//            break;
//        case 2: // Right / 2 b4 3
//            TexCoord = vec3(position.zy, float(textureID));
//            color = vec3(1.0, 1.0, 0.0); // yellow
//            break;
//        case 3: // Left / 3 b4 2
//            TexCoord = vec3(position.zy, float(textureID));
//            color = vec3(0.0, 0.0, 1.0); // blue
//            break;
//        case 4: // Front / 4 b4 0
//            TexCoord = vec3(position.xy, float(textureID));
//            color = vec3(1.0, 0.0, 0.0); // red
//            break;
//        case 5: // Back / 5 b4 1
//            TexCoord = vec3(position.xy, float(textureID));
//            color = vec3(0.0, 1.0, 0.0); // green
//            break;
//    }

    gl_Position = view_projection * vec4(position, 1.0);
}
