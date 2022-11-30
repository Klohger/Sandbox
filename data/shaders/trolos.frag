#version 140
    
out vec4 fragColor;
in vec2 texCoords;
uniform sampler2D video;
uniform float video_opacity;
uniform float opacity;

highp float random(highp vec2 coords) {
   return fract(sin(dot(coords.xy, vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
	vec4 texture = texture(video, texCoords);
	
	fragColor =  mix(vec4(vec3(0.0),opacity), texture, video_opacity);
}