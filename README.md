haven't thought of an actual name yet

optimizations used:
  vertex pulling - uses an ssbo instead of a vao/vbo to store data
  greedy meshing - combines faces of voxels if they're the same
  multi-draw - draws all chunks in 1 draw call using glMultiDrawElementsIndirect
  cpu backface culling - don't tell gpu to draw faces that will never be seen by camera
  frustum culling - frustum culling,

optimizations i want to use:
  gpu frustum and occlusion culling
  level of details
  efficient multi-threading for stuff like meshing
