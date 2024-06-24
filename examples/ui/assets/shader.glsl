varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform vec2 u_pos;
uniform vec2 u_size;

void main() {
    v_vt = a_pos;
    vec2 world_pos = u_pos + u_size * a_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * vec3(world_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform float u_ratio;
uniform sampler2D u_texture;
void main() {
    gl_FragColor = texture2D(u_texture, v_vt);
    if (v_vt.y < u_ratio) {
        gl_FragColor *= vec4(1.0, 0.0, 0.0, 1.0);
    }
}
#endif