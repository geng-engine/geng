varying vec2 v_vt;

#ifdef VERTEX_SHADER
uniform ivec2 u_framebuffer_size;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;

attribute vec2 a_vt;
attribute vec2 a_pos;
void main() {
    v_vt = a_vt;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
    // gl_Position.y = -gl_Position.y;
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_cache_texture;
void main() {
    gl_FragColor = u_color;
    gl_FragColor.w *= texture2D(u_cache_texture, v_vt).w;
}
#endif