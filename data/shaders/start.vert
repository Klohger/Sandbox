#version 140

in vec3 verts;
//in vec2 uv_coords;
//in vec3 normals;
uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

void main() {
	gl_Position = proj * view * model * vec4(verts, 1.0);
}