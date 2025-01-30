#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

void main() {
    vec2 rel_pos = fragTexCoord - vec2(0.5);
    if (dot(rel_pos, rel_pos) <= 0.25) {
        finalColor = texture(texture0, fragTexCoord) * fragColor;
    } else {
        discard;
    }
}
