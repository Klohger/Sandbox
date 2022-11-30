#version 140
    
in vec2 texCoords;
out vec4 fragColor;
uniform vec3 color;
in float vertColors;
void main() {

	fragColor = vec4(color * vertColors, 1.0);
}