#version 140

in vec3 verts;
in vec2 uv_coords;
in vec3 normals;
out vec2 texCoords;
uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;
void main() {
	
	texCoords = uv_coords;
	
	gl_Position = proj * view * model * vec4(verts, 1.0);
}