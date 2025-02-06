#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

const float TAU_RECIP = 0.15915494309;

void main()
{
    vec2 coord = fragTexCoord - vec2(0.5);
    float theta = (coord != vec2(0.0))
        ? acos(coord.x/length(coord)) * TAU_RECIP
        : 0.0;
    if (coord.y < 0.0) {
        theta = 1.0 - theta;
    }
    finalColor = vec4(vec3(theta), 0.0);
}
