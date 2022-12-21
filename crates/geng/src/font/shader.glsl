
varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 i_pos;
attribute vec2 i_size;
attribute vec2 i_uv_pos;
attribute vec2 i_uv_size;

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;
void main() {
    v_uv = i_uv_pos + a_pos * i_uv_size;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(i_pos + a_pos * i_size, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;
uniform vec4 u_color;
uniform vec4 u_outline_color;
uniform float u_outline_dist;

float aa(float edge, float x) {
    float w = length(vec2(dFdx(x), dFdy(x)));
    return smoothstep(edge - w, edge + w, x);
}

void main() {
    float dist = (texture2D(u_texture, v_uv).x - 0.5) * 2.0;
    float w = length(vec2(dFdx(dist), dFdy(dist)));
    float inside = aa(0.0, dist);
    float inside_border = aa(-u_outline_dist, dist);
    gl_FragColor = u_color * inside + (1.0 - inside) * (u_outline_color * inside_border + vec4(u_outline_color.xyz, 0.0) * (1.0 - inside_border));
}
#endif