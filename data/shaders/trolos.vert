#version 140

in vec3 verts;
in vec2 uv_coords;
out vec2 texCoords;
//in vec3 normals;
//uniform mat4 proj;
//uniform mat4 view;
//uniform mat4 model;

void main() {
	texCoords = uv_coords;
	gl_Position = vec4(verts.xy, -1.0, 1.0);
}