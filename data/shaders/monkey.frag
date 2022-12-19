#version 140
out vec4 fragColor;
uniform vec3 color;
noperspective in float vertColors;
void main() {

	fragColor = vec4(color * vertColors, 1.0);
}