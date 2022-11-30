#version 140

in vec3 verts;
//in vec2 uv_coords;
//in vec3 normals;
//uniform mat4 model;
uniform float scale;
uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;
uniform mat4 model2;

void main() {
	vec4 position1 = proj * view * model * vec4(verts, 1.0);
	vec4 position2 = proj * view * model2 * vec4(verts, 1.0);
	gl_Position = mix(position1,position2,((sin(scale) + 1.0) * 0.5));
}