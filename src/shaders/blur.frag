#version 330

const float STDEV_DEFAULT = 5.0;
const float PI_INV = 1.0 / 3.14159265;

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform vec2 size = vec2(480, 480);
uniform float constant = 1.0 / (2.0 * STDEV_DEFAULT * STDEV_DEFAULT);

out vec4 finalColor;

float gaussian(vec2 p) {
    return PI_INV * constant * exp(-dot(p, p) * constant);
}

void main() {
    vec2 px = 1.0 / size;

    finalColor = vec4(0.0);
    for (int i = -10; i <= 10; ++i) {
        for (int j = -10; j <= 10; ++j) {
            vec2 coord = clamp(vec2(i, j) * px + fragTexCoord, 0.0, 1.0);
            float weight = gaussian(coord - fragTexCoord);
            finalColor += texture(texture0, coord) * colDiffuse * fragColor * weight;
        }
    }
}
