varying vec2 v_uv;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec2 i_pos;
attribute vec2 i_size;
attribute vec2 i_uv_pos;
attribute vec2 i_uv_size;

uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    v_uv = i_uv_pos + a_pos * i_uv_size;
    vec3 pos = u_projection_matrix * u_view_matrix * vec3(i_pos + a_pos * i_size, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;

float aa(float edge, float x) {
    float w = length(vec2(dFdx(x), dFdy(x)));
    return smoothstep(edge - w, edge + w, x);
}

void main() {
    vec4 sample = texture2D(u_texture, v_uv);
    float dist = (sample.x - 0.5) * 2.0;
    float w = length(vec2(dFdx(dist), dFdy(dist)));
    float inside = aa(0.0, dist);
    float inside_border = aa(-0.15, dist);
    vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 border_color = vec4(1.0, 1.0, 1.0, 1.0);
    gl_FragColor = color * inside + (1.0 - inside) * (border_color * inside_border + vec4(border_color.xyz, 0.0) * (1.0 - inside_border));
}
#endif