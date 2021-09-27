#define PI 3.1415926535897932384626433832795

vec2 rotate(vec2 v, float a)
{
    float s = sin(a);
    float c = cos(a);
    mat2 m = mat2(c, s, -s, c);
    return m * v;
}

mat4 rotationMatrix(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(oc * axis.x * axis.x + c, oc * axis.x * axis.y + axis.z * s, oc * axis.z * axis.x - axis.y * s, 0.0,
        oc * axis.x * axis.y - axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z + axis.x * s, 0.0,
        oc * axis.z * axis.x + axis.y * s, oc * axis.y * axis.z - axis.x * s, oc * axis.z * axis.z + c, 0.0,
        0.0, 0.0, 0.0, 1.0);
}

mat4 translationMatrix(vec3 v)
{
    return mat4(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, v.x, v.y, v.z, 1.0);
}

mat4 translationMatrix(float x, float y, float z)
{
    return translationMatrix(vec3(x, y, z));
}

mat4 scaleMatrix(vec3 v)
{
    return mat4(v.x, 0.0, 0.0, 0.0, 0.0, v.y, 0.0, 0.0, 0.0, 0.0, v.z, 0.0, 0.0, 0.0, 0.0, 1.0);
}

mat4 scaleMatrix(float s)
{
    return scaleMatrix(vec3(s, s, s));
}

vec3 rotate(vec3 v, vec3 axis, float angle)
{
    mat4 m = rotationMatrix(axis, angle);
    return (m * vec4(v, 1.0)).xyz;
}

float G(vec2 pos, float s)
{
    float sq2 = 2.0 * s * s;
    return exp(-dot(pos, pos) / sq2) / (PI * sq2);
}