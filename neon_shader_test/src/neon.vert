#version 330 core
layout (location = 0) in vec3 aPos;
// Add any vertex-related data here, if needed (e.g., UV coordinates)
out vec2 fragCoord;
void main() {
    // Vertex position transformations (e.g., projection matrix)
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); // Placeholder
    fragCoord = vec2(aPos.x, aPos.y);
}
