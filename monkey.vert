#version 140

in vec3 verts;
in vec2 uv_coords;
in vec3 normals;
out vec2 texCoords;
out float vertColors;
uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;
void main() {
	
	mat4 rotation = view * model;
	rotation = mat4(
		rotation[0][0], rotation[0][1], rotation[0][2], 0,
		rotation[1][0], rotation[1][1], rotation[1][2], 0,
		rotation[2][0], rotation[2][1], rotation[2][2], 0,
		0,              0,              0,              1);
	vec4 position = view * model * vec4(verts, 1.0);
	vertColors = 
		1 - dot(
			normalize(position),
			rotation * vec4(normals,1.0)		
		);
	
	texCoords = uv_coords;
	gl_Position = proj * position;
}