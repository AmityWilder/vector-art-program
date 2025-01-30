#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

uniform vec2 p0 = vec2(0.0);
uniform vec2 p1 = vec2(1.0);
uniform vec2 delta = p1 - p0;
uniform vec2 delta_len_sqr = dot(delta, delta);

void main() {
    vec2 p_delta = fragTexCoord - vec2(0.5);
    if (dot(rel_pos, rel_pos) > 0.25) {
        discard;
    }
    finalColor = texture(texture0, fragTexCoord) * colDiffuse * fragColor;
}
