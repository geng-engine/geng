#define PI 3.1415926535897932384626433832795

vec2 rotate(vec2 v, float a) {
	float s = sin(a);
	float c = cos(a);
	mat2 m = mat2(c, -s, s, c);
	return m * v;
}

float G(vec2 pos, float s) {
    float sq2 = 2.0 * s * s;
    return exp(-dot(pos, pos) / sq2) / (PI * sq2);
}