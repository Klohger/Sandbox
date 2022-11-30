#version 140
    
in vec2 texCoords;
out vec4 fragColor;
in float vertColors;
void main() {

	fragColor = vec4(vec3(vertColors), 1.0);
}